use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use parser::ParsedHeight;
use priority_queue::DoublePriorityQueue;

pub fn part1(data: String) -> String {
    let parsed = parser::parse(&data);
    let (heightmap, start, end) = build_heightmap(parsed);
    let graph = build_graph(&heightmap);
    let (shortest_dist, _shortest_prev) = graph.shortest_paths_from(start);

    shortest_dist.get(&end).unwrap().to_string()
}

pub fn part2(_data: String) -> String {
    panic!("not implemented")
}

const START_HEIGHT: u8 = 0;
const END_HEIGHT: u8 = 25;

fn build_heightmap(parsed: Vec<Vec<ParsedHeight>>) -> (HeightMap, Coord, Coord) {
    let width = parsed[0].len();
    let height = parsed.len();
    let mut heightmap = HeightMap::new(width, height);

    let mut start: Option<Coord> = None;
    let mut end: Option<Coord> = None;

    for (y, row) in parsed.iter().enumerate() {
        for (x, parsed_h) in row.iter().enumerate() {
            let coord = Coord(x, y);
            match parsed_h {
                ParsedHeight::Start => match start {
                    Some(c) => panic!("start already set to x:{} y:{}", c.x(), c.y()),
                    None => {
                        start = Some(coord);
                        heightmap.push(START_HEIGHT);
                    }
                },
                ParsedHeight::End => match end {
                    Some(c) => panic!("end already set to x:{} y:{}", c.x(), c.y()),
                    None => {
                        end = Some(coord);
                        heightmap.push(END_HEIGHT);
                    }
                },
                ParsedHeight::Elevation(h) => heightmap.push(*h),
            }
        }
    }
    (heightmap, start.unwrap(), end.unwrap())
}

fn build_graph(hm: &HeightMap) -> Graph<Coord> {
    let mut g = Graph::new();
    for x in 0..hm.width {
        for y in 0..hm.height {
            let coord = Coord(x, y);
            g.push_vertex(coord);

            let h = hm.get(x, y).unwrap();

            for (n_coord, n_h) in hm.neighbours(x, y).iter() {
                if *n_h <= h + 1 {
                    g.push_edge(coord, *n_coord, 1);
                }
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

struct HeightMap {
    hm: Vec<Height>,
    width: usize,
    height: usize,
}
impl HeightMap {
    fn new(width: usize, height: usize) -> Self {
        Self {
            hm: vec![],
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&Height> {
        self.hm.get(x + y * self.width)
    }

    fn push(&mut self, h: Height) -> () {
        self.hm.push(h);
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<(Coord, Height)> {
        let mut ns = vec![];

        // above
        if y > 0 {
            ns.push((Coord(x, y - 1), *self.get(x, y - 1).unwrap()));
        }

        // below
        if y < self.height - 1 {
            ns.push((Coord(x, y + 1), *self.get(x, y + 1).unwrap()));
        }

        // left
        if x > 0 {
            ns.push((Coord(x - 1, y), *self.get(x - 1, y).unwrap()));
        }

        // right
        if x < self.width - 1 {
            ns.push((Coord(x + 1, y), *self.get(x + 1, y).unwrap()));
        }

        ns
    }
}

type Height = u8;

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

    fn shortest_paths_from(&self, from: T) -> (HashMap<T, u32>, HashMap<T, Option<T>>) {
        let mut dist: HashMap<T, u32> = HashMap::new();
        let mut prev: HashMap<T, Option<T>> = HashMap::new();

        let mut queue = DoublePriorityQueue::new();

        for v in self.vertices.iter() {
            let v_dist = if *v == from { 0 } else { u32::MAX };
            dist.insert(*v, v_dist);
            queue.push(*v, v_dist);
            prev.insert(*v, None);
        }

        loop {
            match queue.pop_min() {
                None => break,
                Some((u, _priority)) => match self.edges.get(&u) {
                    None => (),
                    Some(edges) => {
                        for (v, weight) in edges.iter() {
                            let dist_u = dist.get(&u).unwrap();
                            let dist_v = dist.get(&v).unwrap();
                            let alt = if *dist_u == u32::MAX {
                                u32::MAX
                            } else {
                                dist_u + weight
                            };
                            if alt < *dist_v {
                                dist.insert(*v, alt);
                                prev.insert(*v, Some(u));
                                queue.change_priority(v, alt);
                            }
                        }
                    }
                },
            }
        }

        (dist, prev)
    }
}

mod parser {
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
}
