use std::collections::HashSet;

pub fn part1(data: String) -> String {
    let paths = parse(&data);

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

    let mut units = 0;
    loop {
        match flow_sand(Coord(500, 0), &filled, deepest_y) {
            SandMove::LostToAbyss => break,
            SandMove::AtRest(grain) => filled.insert(grain),
        };
        units += 1;
    }

    units.to_string()
}

pub fn part2(_data: String) -> String {
    panic!("not implemented");
}

type Sandgrain = Coord;
type Fillmap = HashSet<Sandgrain>;

enum SandMove {
    LostToAbyss,
    AtRest(Coord),
}

fn flow_sand(grain: Sandgrain, filled: &Fillmap, deepest: u32) -> SandMove {
    let mut g = grain.clone();

    loop {
        let below = Coord(g.x(), g.y() + 1);
        if below.y() > deepest {
            return SandMove::LostToAbyss;
        }
        if !filled.contains(&below) {
            g = below;
            continue;
        }

        let left = Coord(g.x() - 1, g.y() + 1);
        if !filled.contains(&left) {
            g = left;
            continue;
        }

        let right = Coord(g.x() + 1, g.y() + 1);
        if !filled.contains(&right) {
            g = right;
            continue;
        }

        return SandMove::AtRest(g);
    }
}

#[derive(Debug, PartialEq)]
struct Path(Vec<Coord>);

impl<'a> Path {
    fn coords(&'a self) -> Vec<Coord> {
        let mut segments = vec![];
        for i in 0..self.0.len() - 1 {
            let from = &self.0[i];
            let to = &self.0[i + 1];
            segments.push(Segment { from, to });
        }
        segments
            .iter()
            .flat_map(|s| s.coords())
            .collect::<Vec<Coord>>()
    }
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Ord, Eq, Hash)]
struct Coord(u32, u32);

impl Coord {
    fn x(&self) -> u32 {
        self.0
    }

    fn y(&self) -> u32 {
        self.1
    }
}

struct Segment<'a> {
    from: &'a Coord,
    to: &'a Coord,
}

impl<'a> Segment<'a> {
    fn coords(&self) -> SegmentCoordsIterator {
        let mut coords = vec![&self.from, &self.to];
        coords.sort();
        let start = coords[0];
        let end = coords[1];
        let current = (*start).clone();
        let dir = if start.x() != end.x() && start.y() == end.y() {
            Dir::Horizontal
        } else if start.x() == end.x() && start.y() != end.y() {
            Dir::Vertical
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

enum Dir {
    Horizontal,
    Vertical,
}

struct SegmentCoordsIterator<'a> {
    current: Coord,
    end: &'a Coord,
    dir: Dir,
    finished: bool,
}

impl<'a> Iterator for SegmentCoordsIterator<'a> {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let prev_current = self.current.clone();

        if &self.current == self.end {
            self.finished = true;
        } else {
            self.current = match self.dir {
                Dir::Horizontal => Coord(self.current.x() + 1, self.current.y()),
                Dir::Vertical => Coord(self.current.x(), self.current.y() + 1),
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

fn coord(s: &str) -> IResult<&str, Coord> {
    let p = separated_pair(u32, tag(","), u32);
    let mut p = map(p, |(x, y)| Coord(x, y));
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
                Path(vec![Coord(498, 4), Coord(498, 6), Coord(496, 6)]),
                Path(vec![
                    Coord(503, 4),
                    Coord(502, 4),
                    Coord(502, 9),
                    Coord(494, 9)
                ])
            ]
        );
    }
}
