use std::collections::HashSet;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::u32, combinator::map,
    multi::separated_list0, sequence::separated_pair, IResult,
};

use crate::coord::{Coord, Direction, ICoord};

pub fn part1(data: String) -> String {
    run(data, 2).to_string()
}

pub fn part2(data: String) -> String {
    run(data, 10).to_string()
}

fn run(data: String, rope_length: usize) -> usize {
    let mut rope = Rope::new(rope_length);
    let mut tail_visits: HashSet<Coord<i32>> = HashSet::new();
    for instruction in parse(&data) {
        for _ in 0..instruction.size {
            rope.mv_head(&instruction.dir);
            tail_visits.insert(rope.tail().clone());
        }
    }
    tail_visits.len()
}

struct Rope {
    nodes: Vec<Coord<i32>>,
}

impl Rope {
    fn new(length: usize) -> Self {
        let mut nodes = Vec::with_capacity(length);
        for _ in 0..length {
            nodes.push(Coord::new(0, 0));
        }
        Rope { nodes }
    }

    fn mv_head(&mut self, dir: &Direction) {
        let mut prev_node = self.nodes[0].mv(dir);
        self.nodes[0] = prev_node;
        for i in 1..self.nodes.len() {
            let node = self.nodes[i];
            match chase(&node, &prev_node) {
                Some(new_node) => {
                    self.nodes[i] = new_node;
                    prev_node = new_node;
                }
                None => prev_node = node,
            }
        }
    }

    fn tail(&self) -> &Coord<i32> {
        &self.nodes[self.nodes.len() - 1]
    }
}

fn chase(coord: &Coord<i32>, other: &Coord<i32>) -> Option<Coord<i32>> {
    let x_diff = coord.x().abs_diff(other.x());
    let y_diff = coord.y().abs_diff(other.y());

    if x_diff <= 1 && y_diff <= 1 {
        //already touching
        None
    } else if x_diff > y_diff {
        // previous moved away horizontally
        let d_x = if coord.x() < other.x() { -1 } else { 1 };
        Some(Coord::new(other.x() + d_x, other.y()))
    } else if x_diff < y_diff {
        // previous moved away vertically
        let d_y = if coord.y() < other.y() { -1 } else { 1 };
        Some(Coord::new(other.x(), other.y() + d_y))
    } else if x_diff == y_diff {
        // previous moved away diagonally
        let d_x = if coord.x() < other.x() { -1 } else { 1 };
        let d_y = if coord.y() < other.y() { -1 } else { 1 };
        Some(Coord::new(other.x() + d_x, other.y() + d_y))
    } else {
        //shouldn't get here
        panic!("x_diff: {x_diff} y_diff: {y_diff}")
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    dir: Direction,
    size: MovementSize,
}

type MovementSize = u32;

fn parse(input: &str) -> Vec<Instruction> {
    let (_rest, instructions) = instructions_parser(input).unwrap();
    instructions
}

fn instructions_parser(input: &str) -> IResult<&str, Vec<Instruction>> {
    let mut parser = separated_list0(tag("\n"), instruction_parser);
    parser(input)
}

fn instruction_parser(input: &str) -> IResult<&str, Instruction> {
    let mut parser = map(separated_pair(dir_parser, tag(" "), u32), |(d, s)| {
        Instruction { dir: d, size: s }
    });
    parser(input)
}

fn dir_parser(input: &str) -> IResult<&str, Direction> {
    use Direction::*;
    let l = map(tag("L"), |_| L);
    let r = map(tag("R"), |_| R);
    let u = map(tag("U"), |_| U);
    let d = map(tag("D"), |_| D);
    let mut parser = alt((l, r, u, d));
    parser(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Direction::*;

    #[test]
    fn it_parses_instruction() {
        assert_eq!(
            instruction_parser("R 4").unwrap(),
            ("", Instruction { dir: R, size: 4 })
        );
        assert_eq!(
            instruction_parser("U 4").unwrap(),
            ("", Instruction { dir: U, size: 4 })
        );
        assert_eq!(
            instruction_parser("L 3").unwrap(),
            ("", Instruction { dir: L, size: 3 })
        );
        assert_eq!(
            instruction_parser("D 11").unwrap(),
            ("", Instruction { dir: D, size: 11 })
        );

        assert_eq!(
            instruction_parser("").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"\", code: Tag }"
        );
        assert_eq!(
            instruction_parser("R").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"\", code: Tag }"
        );
        assert_eq!(
            instruction_parser("R ").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"\", code: Digit }"
        );
        assert_eq!(
            instruction_parser("R -123").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"-123\", code: Digit }"
        );
        assert_eq!(
            instruction_parser("R foobar").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"foobar\", code: Digit }"
        );
        assert_eq!(
            instruction_parser("Z 12").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"Z 12\", code: Tag }"
        );
    }

    #[test]
    fn it_parses_instructions() {
        let data = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(
            instructions_parser(data),
            Ok((
                "",
                vec![
                    Instruction { dir: R, size: 4 },
                    Instruction { dir: U, size: 4 },
                    Instruction { dir: L, size: 3 },
                    Instruction { dir: D, size: 1 },
                    Instruction { dir: R, size: 4 },
                    Instruction { dir: D, size: 1 },
                    Instruction { dir: L, size: 5 },
                    Instruction { dir: R, size: 2 }
                ]
            ))
        );
    }

    #[test]
    fn it_chases_right() {
        let head = Coord::new(3, 0);
        let tail = Coord::new(1, 0);
        assert_eq!(chase(&tail, &head), Some(Coord::new(2, 0)));

        let head = Coord::new(-1, 0);
        let tail = Coord::new(-3, 0);
        assert_eq!(chase(&tail, &head), Some(Coord::new(-2, 0)));
    }

    #[test]
    fn it_chase_left() {
        let head = Coord::new(1, 0);
        let tail = Coord::new(3, 0);
        assert_eq!(chase(&tail, &head), Some(Coord::new(2, 0)));

        let head = Coord::new(-3, 0);
        let tail = Coord::new(-1, 0);
        assert_eq!(chase(&tail, &head), Some(Coord::new(-2, 0)));
    }

    #[test]
    fn it_chases_up() {
        let head = Coord::new(0, 3);
        let tail = Coord::new(0, 1);
        assert_eq!(chase(&tail, &head), Some(Coord::new(0, 2)));

        let head = Coord::new(0, -1);
        let tail = Coord::new(0, -3);
        assert_eq!(chase(&tail, &head), Some(Coord::new(0, -2)));
    }

    #[test]
    fn it_chases_down() {
        let head = Coord::new(0, 1);
        let tail = Coord::new(0, 3);
        assert_eq!(chase(&tail, &head), Some(Coord::new(0, 2)));

        let head = Coord::new(0, -3);
        let tail = Coord::new(0, -1);
        assert_eq!(chase(&tail, &head), Some(Coord::new(0, -2)));
    }

    #[test]
    fn it_doesnt_chase_if_touching() {
        let head = Coord::new(2, 3);
        assert_eq!(chase(&Coord::new(1, 2), &head), None); // above left
        assert_eq!(chase(&Coord::new(2, 2), &head), None); // above
        assert_eq!(chase(&Coord::new(3, 2), &head), None); // above right
        assert_eq!(chase(&Coord::new(1, 3), &head), None); // left
        assert_eq!(chase(&Coord::new(2, 3), &head), None); // same position
        assert_eq!(chase(&Coord::new(3, 3), &head), None); // right
        assert_eq!(chase(&Coord::new(1, 4), &head), None); // below left
        assert_eq!(chase(&Coord::new(2, 4), &head), None); // below
        assert_eq!(chase(&Coord::new(3, 4), &head), None); // below right
    }
}
