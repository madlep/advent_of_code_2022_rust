use crate::coord::{Coord, ICoord, Orientation};
use std::collections::HashSet;

pub fn part1(data: String) -> String {
    let paths = parse(&data);
    let (mut filled, deepest_y) = build_scan(paths);

    let mut units = 0;
    loop {
        match flow_sand(Sandgrain::new(500, 0), &filled, Bottom::Abyss(deepest_y)) {
            SandMove::LostToAbyss => break,
            SandMove::AtRest(grain) => filled.insert(grain),
        };
        units += 1;
    }

    units.to_string()
}

pub fn part2(data: String) -> String {
    let paths = parse(&data);
    let (mut filled, deepest_y) = build_scan(paths);

    let mut units = 1;
    loop {
        match flow_sand(
            Sandgrain::new(500, 0),
            &filled,
            Bottom::Floor(deepest_y + 2),
        ) {
            SandMove::LostToAbyss => panic!("shouldn't lose sand"),
            SandMove::AtRest(grain) => {
                if grain.y() == 0 && grain.x() == 500 {
                    break;
                } else {
                    filled.insert(grain);
                }
            }
        };
        units += 1;
    }

    units.to_string()
}

fn build_scan(paths: Vec<Path>) -> (Fillmap, u32) {
    let mut deepest_y = 0;
    let mut filled: Fillmap = Fillmap::new();
    for path in paths.iter() {
        for coord in path.coords() {
            if coord.y() > deepest_y {
                deepest_y = coord.y()
            }
            filled.insert(coord);
        }
    }
    (filled, deepest_y)
}

type Sandgrain = Coord<u32>;
type Fillmap = HashSet<Sandgrain>;

enum SandMove {
    LostToAbyss,
    AtRest(Sandgrain),
}

enum Bottom {
    Abyss(u32),
    Floor(u32),
}

fn flow_sand(grain: Sandgrain, filled: &Fillmap, deepest: Bottom) -> SandMove {
    let mut g = grain.clone();

    loop {
        let below = Coord::new(g.x(), g.y() + 1);
        match deepest {
            Bottom::Abyss(d) => {
                if below.y() > d {
                    return SandMove::LostToAbyss;
                }
            }
            Bottom::Floor(d) => {
                if below.y() == d {
                    return SandMove::AtRest(g);
                }
            }
        }
        if !filled.contains(&below) {
            g = below;
            continue;
        }

        let left = Coord::new(g.x() - 1, g.y() + 1);
        if !filled.contains(&left) {
            g = left;
            continue;
        }

        let right = Coord::new(g.x() + 1, g.y() + 1);
        if !filled.contains(&right) {
            g = right;
            continue;
        }

        return SandMove::AtRest(g);
    }
}

#[derive(Debug, PartialEq)]
struct Path(Vec<Coord<u32>>);

impl<'a> Path {
    fn coords(&'a self) -> Vec<Coord<u32>> {
        let mut segments = vec![];
        for i in 0..self.0.len() - 1 {
            let from = &self.0[i];
            let to = &self.0[i + 1];
            segments.push(Segment { from, to });
        }
        segments
            .iter()
            .flat_map(|s| s.coords())
            .collect::<Vec<Coord<u32>>>()
    }
}

struct Segment<'a> {
    from: &'a Coord<u32>,
    to: &'a Coord<u32>,
}

impl<'a> Segment<'a> {
    fn coords(&self) -> SegmentCoordsIterator {
        let mut coords = vec![&self.from, &self.to];
        coords.sort();
        let start = coords[0];
        let end = coords[1];
        let current = (*start).clone();
        let dir = if start.x() != end.x() && start.y() == end.y() {
            Orientation::Horizontal
        } else if start.x() == end.x() && start.y() != end.y() {
            Orientation::Vertical
        } else {
            panic!("coords not aligned horizontally or vertically");
        };

        SegmentCoordsIterator {
            current,
            end,
            dir,
            finished: false,
        }
    }
}

struct SegmentCoordsIterator<'a> {
    current: Coord<u32>,
    end: &'a Coord<u32>,
    dir: Orientation,
    finished: bool,
}

impl<'a> Iterator for SegmentCoordsIterator<'a> {
    type Item = Coord<u32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let prev_current = self.current.clone();

        if &self.current == self.end {
            self.finished = true;
        } else {
            self.current = match self.dir {
                Orientation::Horizontal => Coord::new(self.current.x() + 1, self.current.y()),
                Orientation::Vertical => Coord::new(self.current.x(), self.current.y() + 1),
            };
        }

        Some(prev_current)
    }
}

fn parse(s: &str) -> Vec<Path> {
    let (_rest, paths) = paths(s).unwrap();
    paths
}

use nom::{
    bytes::complete::tag, character::complete::u32, combinator::map, multi::separated_list0,
    sequence::separated_pair, IResult,
};

fn paths(s: &str) -> IResult<&str, Vec<Path>> {
    separated_list0(tag("\n"), path)(s)
}

fn path(s: &str) -> IResult<&str, Path> {
    let p = separated_list0(tag(" -> "), coord);
    let mut p = map(p, |coords| Path(coords));
    p(s)
}

fn coord(s: &str) -> IResult<&str, Coord<u32>> {
    let p = separated_pair(u32, tag(","), u32);
    let mut p = map(p, |(x, y)| Coord::new(x, y));
    p(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_parses_paths() {
        let data = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(
            parse(data),
            vec![
                Path(vec![
                    Coord::new(498, 4),
                    Coord::new(498, 6),
                    Coord::new(496, 6)
                ]),
                Path(vec![
                    Coord::new(503, 4),
                    Coord::new(502, 4),
                    Coord::new(502, 9),
                    Coord::new(494, 9)
                ])
            ]
        );
    }
}
