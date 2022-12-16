use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use priority_queue::DoublePriorityQueue;

pub fn part1(data: String) -> String {
    let parsed = parse(&data);
    let (heightmap, start, end) = build_heightmap(parsed);

    // paths from start -> every other square
    let graph = build_graph(&heightmap, |height, neighbour_height| {
        height + 1 >= neighbour_height
    });
    let shortest_paths = graph.shortest_paths_from(start);

    shortest_paths.get(&end).unwrap().to_string()
}

pub fn part2(data: String) -> String {
    let parsed = parse(&data);
    let (heightmap, _start, end) = build_heightmap(parsed);

    // paths from end -> every other square
    let reverse_graph = build_graph(&heightmap, |height, neighbour_height| {
        height <= neighbour_height + 1
    });
    let reverse_shortest_paths = reverse_graph.shortest_paths_from(end);

    heightmap
        .iter()
        .filter_map(|(c, h)| {
            if *h == START_HEIGHT {
                Some(reverse_shortest_paths.get(c).unwrap())
            } else {
                None
            }
        })
        .min()
        .unwrap()
        .to_string()
}

const START_HEIGHT: u8 = 0;
const END_HEIGHT: u8 = 25;
const STEP_WEIGHT: u32 = 1;

fn build_heightmap(parsed: Vec<Vec<ParsedHeight>>) -> (HeightMap, Coord, Coord) {
    let mut heightmap = HeightMap::new();
    let mut start: Option<Coord> = None;
    let mut end: Option<Coord> = None;

    for (y, row) in parsed.iter().enumerate() {
        for (x, parsed_h) in row.iter().enumerate() {
            let coord = Coord(x, y);
            let h = match parsed_h {
                ParsedHeight::Start => match start {
                    Some(_) => panic!("start already set"),
                    None => {
                        start = Some(coord);
                        START_HEIGHT
                    }
                },
                ParsedHeight::End => match end {
                    Some(_) => panic!("end already set"),
                    None => {
                        end = Some(coord);
                        END_HEIGHT
                    }
                },
                ParsedHeight::Elevation(h) => *h,
            };
            heightmap.insert(coord, h);
        }
    }
    (heightmap, start.unwrap(), end.unwrap())
}

fn build_graph(
    hm: &HeightMap,
    neighbour_check: impl Fn(Height, NeighbourHeight) -> bool,
) -> Graph<Coord> {
    let mut g: Graph<Coord> = Graph::new();
    for (coord, h) in hm.iter() {
        g.push_vertex(*coord);
        for (n_coord, n_h) in neighbours(hm, *coord).iter() {
            if neighbour_check(*h, *n_h) {
                g.push_edge(*coord, *n_coord, STEP_WEIGHT);
            }
        }
    }
    g
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Coord(usize, usize);

impl Coord {
    fn x(&self) -> usize {
        self.0
    }

    fn y(&self) -> usize {
        self.1
    }
}

type HeightMap = HashMap<Coord, Height>;

fn neighbours(hm: &HeightMap, coord: Coord) -> Vec<(Coord, NeighbourHeight)> {
    let x = coord.x();
    let y = coord.y();

    let mut ns = vec![];
    ns.push(Coord(x, y + 1));
    ns.push(Coord(x + 1, y));
    // guard against panic due to subtracing from 0 for usize
    if y > 0 {
        ns.push(Coord(x, y - 1))
    };
    if x > 0 {
        ns.push(Coord(x - 1, y))
    };

    ns.iter()
        .filter_map(|c| hm.get(c).map(|h| (*c, *h)))
        .collect()
}

type Height = u8;
type NeighbourHeight = u8;

type EdgeWeight = u32;

#[derive(Debug)]
struct Graph<T> {
    vertices: HashSet<T>,
    edges: HashMap<T, HashMap<T, EdgeWeight>>,
}

impl<T: Eq + Hash + Copy + Debug> Graph<T> {
    fn new() -> Self {
        Self {
            vertices: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    fn push_vertex(&mut self, vertex: T) -> () {
        self.vertices.insert(vertex);
    }

    fn push_edge(&mut self, from: T, to: T, weight: EdgeWeight) -> () {
        match self.edges.get_mut(&from) {
            Some(edge_ends) => {
                edge_ends.insert(to, weight);
            }
            None => {
                let mut edge_ends = HashMap::new();
                edge_ends.insert(to, weight);
                self.edges.insert(from, edge_ends);
            }
        }
    }

    fn shortest_paths_from(&self, from: T) -> HashMap<T, u32> {
        let mut dist: HashMap<T, u32> = HashMap::new();
        let mut queue = DoublePriorityQueue::new();
        // big, but still able to add weight so we don't have to guard alt calculation for overflow
        const INFINITY: u32 = u32::MAX / 2;

        for v in self.vertices.iter() {
            let v_dist = if *v == from { 0 } else { INFINITY };
            dist.insert(*v, v_dist);
            queue.push(*v, v_dist);
        }

        while let Some((u, _priority)) = queue.pop_min() {
            if let Some(edges) = self.edges.get(&u) {
                let dist_u = *dist.get(&u).unwrap();
                for (v, weight) in edges.iter() {
                    let dist_v = dist.get(&v).unwrap();
                    let alt = dist_u + weight;
                    if alt < *dist_v {
                        dist.insert(*v, alt);
                        queue.change_priority(v, alt);
                    }
                }
            }
        }

        dist
    }
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::satisfy,
    combinator::{map, value},
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Clone, Debug)]
pub enum ParsedHeight {
    Start,
    End,
    Elevation(u8),
}

const ASCII_OFFSET: u8 = 97;

pub fn parse(s: &str) -> Vec<Vec<ParsedHeight>> {
    let (_rest, hm) = heightmap(s).unwrap();
    hm
}
fn heightmap(s: &str) -> IResult<&str, Vec<Vec<ParsedHeight>>> {
    separated_list1(tag("\n"), heightmap_row)(s)
}

fn heightmap_row(s: &str) -> IResult<&str, Vec<ParsedHeight>> {
    many1(height)(s)
}

fn height(s: &str) -> IResult<&str, ParsedHeight> {
    alt((start, end, elevation))(s)
}

fn start(s: &str) -> IResult<&str, ParsedHeight> {
    let mut p = value(ParsedHeight::Start, tag("S"));
    p(s)
}

fn end(s: &str) -> IResult<&str, ParsedHeight> {
    let mut p = value(ParsedHeight::End, tag("E"));
    p(s)
}

fn elevation(s: &str) -> IResult<&str, ParsedHeight> {
    let p = satisfy(|c| c >= 'a' && c <= 'z');
    let mut p = map(p, |c| ParsedHeight::Elevation((c as u8) - ASCII_OFFSET));
    p(s)
}
