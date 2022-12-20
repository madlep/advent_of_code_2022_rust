use rpds::HashTrieMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

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

const TOTAL_MINUTES: u32 = 30;

pub fn part1(data: String) -> String {
    let rooms = parse(&data);

    let flows = rooms.values().fold(HashTrieMap::new(), |acc, room| {
        acc.insert(room.valve_label, room.flow_rate)
    });

    let best_found = Rc::new(RefCell::new(0_u32));

    let root = SearchState {
        score: 0,
        current_valve: hash_valve_label(('A', 'A')),
        flows,
        rooms: Rc::new(rooms),
        remaining_minutes: TOTAL_MINUTES,
        best_found: (&best_found).clone(),
    };

    let best = search(root);

    best.to_string()
}

pub fn part2(_data: String) -> String {
    panic!("not implemented");
}

fn search(state: SearchState) -> u32 {
    if reject(&state) {
        return state.best_found.borrow().clone();
    }

    if accept(&state) {
        let score = state.score;
        let mut best = state.best_found.borrow_mut();
        if score > *best {
            *best = score;
        }
        return best.clone();
    }

    let mut best = 0_u32;

    for state in next_states(&state) {
        best = search(state).max(best);
    }

    best
}

fn reject(state: &SearchState) -> bool {
    // if it's impossible to beat the current best score even if we open ALL the remaining valves,
    // then don't bother searching that path.
    // This isn't exhaustive due to not accounting for
    // - moving time
    // - whether it is possible to even get to all the valves in remaining time
    // so it may need to do extra work, but it won't skip any possibilities

    let possible_remaining_score = state
        .flows
        .values()
        .map(|flow| flow * (state.remaining_minutes.max(1) - 1))
        .sum::<u32>();

    let possible_score = state.score + possible_remaining_score;

    possible_score <= *state.best_found.borrow()
}

fn accept(state: &SearchState) -> bool {
    state.remaining_minutes == 0 || state.is_no_more_flows()
}

fn next_states(state: &SearchState) -> Vec<SearchState> {
    let mut states = vec![];
    if !state.is_no_more_flows() {
        let mut g = Graph::new();

        for room in state.rooms.values() {
            g.push_vertex(room.valve_label);
            for path in room.paths.iter() {
                g.push_edge(room.valve_label, *path, 1)
            }
        }
        let shortest_dists = g.shortest_paths_from(&state.current_valve);

        for unopened_valve in state.unopened_valves().iter() {
            if unopened_valve != &state.current_valve {
                states.push(state.go_to_valve_and_open(
                    *unopened_valve,
                    *shortest_dists.get(unopened_valve).unwrap(),
                ));
            }
        }
    }

    states
}

#[derive(Clone, Debug)]
struct SearchState {
    score: u32,
    current_valve: ValveLabel,
    flows: HashTrieMap<ValveLabel, FlowRate>,
    rooms: Rc<HashMap<ValveLabel, Room>>,
    remaining_minutes: u32,
    best_found: Rc<RefCell<u32>>,
}

impl SearchState {
    fn go_to_valve_and_open(&self, valve: ValveLabel, travel_time: u32) -> Self {
        let flow = self.flows.get(&valve).unwrap();

        let new_remaining = if self.remaining_minutes > travel_time {
            self.remaining_minutes - travel_time - 1
        } else {
            0
        };

        Self {
            score: self.score + flow * new_remaining,
            current_valve: valve,
            flows: self.flows.insert(self.current_valve, 0),
            remaining_minutes: new_remaining,
            ..self.clone()
        }
    }

    fn is_no_more_flows(&self) -> bool {
        self.flows_left() == 0
    }

    fn flows_left(&self) -> u32 {
        self.flows.values().sum::<u32>()
    }

    fn unopened_valves(&self) -> Vec<ValveLabel> {
        self.flows
            .iter()
            .filter_map(|(valve_label, flow)| if *flow > 0 { Some(*valve_label) } else { None })
            .collect()
    }
}

fn hash_valve_label(label: (char, char)) -> u64 {
    let mut s = DefaultHasher::new();
    label.hash(&mut s);
    s.finish()
}

type ValveLabel = u64;
type FlowRate = u32;

#[derive(Debug, PartialEq)]
struct Room {
    valve_label: ValveLabel,
    flow_rate: FlowRate,
    paths: Vec<ValveLabel>,
}

fn parse(s: &str) -> HashMap<ValveLabel, Room> {
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
