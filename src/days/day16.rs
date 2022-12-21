use rpds::HashTrieMap;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    character::complete::u32,
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};

use crate::graph::Graph;

const TOTAL_MINUTES: Minute = 30;
const OPEN_TIME: Minute = 1;
const TRAVEL_TIME: Minute = 1;
const NO_FLOW: FlowRate = 0;

pub fn part1(data: String) -> String {
    let current_valve = hash_valve_label(('A', 'A'));
    let rooms = parse(&data);

    let root = SearchState {
        score: NO_FLOW,
        current_valve,
        flows: build_flows(&rooms),
        remaining_minutes: TOTAL_MINUTES,
    };

    root.search(NO_FLOW, &build_shortest_from_to(current_valve, &rooms))
        .to_string()
}

pub fn part2(_data: String) -> String {
    panic!("not implemented");
}

type ValveLabel = u64;
type Minute = u32;
type FlowRate = u32;

type Rooms = HashMap<ValveLabel, Room>;
type PathGraph = Graph<ValveLabel, Minute>;
type ShortestDistFromTo = HashMap<(ValveLabel, ValveLabel), Minute>;

type Flows = HashTrieMap<ValveLabel, FlowRate>;

fn build_flows(rooms: &Rooms) -> Flows {
    rooms.values().fold(Flows::new(), |acc, room| {
        acc.insert(room.valve_label, room.flow_rate)
    })
}

fn build_path_graph(rooms: &Rooms) -> PathGraph {
    let mut path_graph = PathGraph::new();
    for room in rooms.values() {
        path_graph.push_vertex(room.valve_label);
        for path in room.paths.iter() {
            path_graph.push_edge(room.valve_label, *path, TRAVEL_TIME)
        }
    }
    path_graph
}

fn build_shortest_from_to(initial: ValveLabel, rooms: &Rooms) -> ShortestDistFromTo {
    let path_graph = build_path_graph(&rooms);

    let mut valves_with_flow: Vec<ValveLabel> = rooms
        .values()
        .filter_map(|r| {
            if r.flow_rate > 0 {
                Some(r.valve_label)
            } else {
                None
            }
        })
        .collect();
    valves_with_flow.push(initial);

    valves_with_flow.into_iter().fold(
        ShortestDistFromTo::new(),
        |mut shortest_dists_from, from| {
            let shortest_dists_to = path_graph
                .shortest_paths_from(&from)
                .into_iter()
                .map(|(to, dist)| ((from, to), dist));

            shortest_dists_from.extend(shortest_dists_to);
            shortest_dists_from
        },
    )
}

#[derive(Clone, Debug)]
struct SearchState {
    score: FlowRate,
    current_valve: ValveLabel,
    flows: Flows,
    remaining_minutes: Minute,
}

impl SearchState {
    fn search(
        &self,
        best_found: FlowRate,
        shortest_dists_from_to: &ShortestDistFromTo,
    ) -> FlowRate {
        if self.reject(best_found) {
            self.score.max(best_found)
        } else if self.accept() {
            self.score.max(best_found)
        } else {
            self.next_states(shortest_dists_from_to)
                .fold(self.score.max(best_found), |current_best, s| {
                    s.search(current_best, shortest_dists_from_to)
                })
        }
    }

    fn reject(&self, best_found: FlowRate) -> bool {
        // if it's impossible to beat the current best score even if we open ALL the remaining valves,
        // then don't bother searching that path.
        // This isn't exhaustive due to not accounting for
        // - moving time more than one room
        // - whether it is possible to even get to all the valves in remaining time
        // so it may need to do extra work, but it won't skip any possibilities

        let possible_remaining_score = if self.remaining_minutes < TRAVEL_TIME + OPEN_TIME {
            NO_FLOW
        } else {
            self.flows
                .values()
                .map(|flow| flow * (self.remaining_minutes - TRAVEL_TIME - OPEN_TIME))
                .sum::<FlowRate>()
        };

        let possible_score = self.score + possible_remaining_score;

        possible_score <= best_found
    }

    fn accept(&self) -> bool {
        self.remaining_minutes < TRAVEL_TIME + OPEN_TIME || self.is_no_more_flows()
    }

    fn next_states<'a>(
        &'a self,
        shortest_dists_from_to: &'a ShortestDistFromTo,
    ) -> impl Iterator<Item = SearchState> + 'a {
        self.unopened_valves()
            .into_iter()
            .filter_map(|unopened_valve| {
                let travel_time = shortest_dists_from_to[&(self.current_valve, unopened_valve)];

                if unopened_valve != self.current_valve
                    && travel_time + OPEN_TIME < self.remaining_minutes
                {
                    Some(self.go_to_valve_and_open(unopened_valve, travel_time))
                } else {
                    None
                }
            })
    }

    fn go_to_valve_and_open(&self, valve: ValveLabel, travel_time: Minute) -> Self {
        let flow = self.flows[&valve];

        let new_remaining = self.remaining_minutes - travel_time - OPEN_TIME;

        Self {
            score: self.score + flow * new_remaining,
            current_valve: valve,
            flows: self.flows.insert(self.current_valve, NO_FLOW),
            remaining_minutes: new_remaining,
            ..self.clone()
        }
    }

    fn is_no_more_flows(&self) -> bool {
        self.flows_left() == NO_FLOW
    }

    fn flows_left(&self) -> FlowRate {
        self.flows.values().sum::<FlowRate>()
    }

    fn unopened_valves(&self) -> Vec<ValveLabel> {
        self.flows
            .iter()
            .filter_map(|(valve_label, flow)| {
                if *flow > NO_FLOW {
                    Some(*valve_label)
                } else {
                    None
                }
            })
            .collect()
    }
}

fn hash_valve_label(label: (char, char)) -> ValveLabel {
    let mut s = DefaultHasher::new();
    label.hash(&mut s);
    s.finish()
}

#[derive(Debug, PartialEq)]
struct Room {
    valve_label: ValveLabel,
    flow_rate: FlowRate,
    paths: Vec<ValveLabel>,
}

fn parse(s: &str) -> Rooms {
    let (_rest, rooms) = rooms(s).unwrap();

    rooms
        .into_iter()
        .map(|room| (room.valve_label, room))
        .collect()
}

fn rooms(s: &str) -> IResult<&str, Vec<Room>> {
    separated_list0(tag("\n"), room)(s)
}

//Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
fn room(s: &str) -> IResult<&str, Room> {
    let tunnels_sep = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ));
    let p = tuple((
        preceded(tag("Valve "), valve_label),
        preceded(tag(" has flow rate="), flow_rate),
        preceded(tunnels_sep, valve_label_list),
    ));
    let mut p = map(p, |(valve_label, flow_rate, paths)| Room {
        valve_label,
        flow_rate,
        paths,
    });
    p(s)
}

//AA
fn valve_label(s: &str) -> IResult<&str, ValveLabel> {
    let p = tuple((anychar, anychar));
    let mut p = map(p, hash_valve_label);
    p(s)
}

//0
fn flow_rate(s: &str) -> IResult<&str, FlowRate> {
    u32(s)
}

//DD, II, BB
fn valve_label_list(s: &str) -> IResult<&str, Vec<ValveLabel>> {
    separated_list0(tag(", "), valve_label)(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_input_line() {
        assert_eq!(
            room("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB").unwrap(),
            (
                "",
                Room {
                    valve_label: hash_valve_label(('A', 'A')),
                    flow_rate: 0,
                    paths: vec![
                        hash_valve_label(('D', 'D')),
                        hash_valve_label(('I', 'I')),
                        hash_valve_label(('B', 'B'))
                    ]
                }
            )
        )
    }

    #[test]
    fn it_parses_input_line_with_singular_tunnel() {
        assert_eq!(
            room("Valve HH has flow rate=22; tunnel leads to valve GG").unwrap(),
            (
                "",
                Room {
                    valve_label: hash_valve_label(('H', 'H')),
                    flow_rate: 22,
                    paths: vec![hash_valve_label(('G', 'G'))]
                }
            )
        )
    }
}
