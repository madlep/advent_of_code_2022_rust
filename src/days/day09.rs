use std::collections::HashSet;
use std::hash::Hash;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::u8, combinator::map,
    multi::separated_list0, sequence::separated_pair, IResult,
};

pub fn part1(data: String) -> String {
    let mut rope = Rope::new();
    let mut tail_visits: HashSet<Coord> = HashSet::new();
    for instruction in parse(&data) {
        for _ in 0..instruction.size {
            rope.mv_head(&instruction.dir);
            tail_visits.insert(rope.tail);
        }
    }
    tail_visits.len().to_string()
}

pub fn part2(_data: String) -> String {
    panic!("not implemented")
}

struct Rope {
    head: Coord,
    tail: Coord,
}

impl Rope {
    fn new() -> Self {
        Rope {
            head: Coord(0, 0),
            tail: Coord(0, 0),
        }
    }

    fn mv_head(&mut self, dir: &Direction) {
        self.head = self.head.mv(dir);
        match self.tail.catch(&self.head) {
            Some(new_tail) => self.tail = new_tail,
            None => (),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Coord(i32, i32);
impl Coord {
    fn x(&self) -> i32 {
        self.0
    }
    fn y(&self) -> i32 {
        self.1
    }
    fn mv(&self, dir: &Direction) -> Self {
        use Direction::*;
        match dir {
            L => Self(self.x() - 1, self.y()),
            R => Self(self.x() + 1, self.y()),
            U => Self(self.x(), self.y() - 1),
            D => Self(self.x(), self.y() + 1),
        }
    }

    fn catch(&self, other: &Self) -> Option<Self> {
        let x_diff = self.x().abs_diff(other.x());
        let y_diff = self.y().abs_diff(other.y());

        if x_diff <= 1 && y_diff <= 1 {
            //already touching
            None
        } else if x_diff > 1 && y_diff > 1 {
            // shouldn't get here
            panic!("coord too far to catch. x_diff: {x_diff}, y_diff: {y_diff}");
        } else if x_diff > 1 {
            // H moved away horizontally,
            let d_x = if self.x() < other.x() { 1 } else { -1 };
            Some(Self(self.x() + d_x, other.y()))
        } else {
            // y_diff

            let d_y = if self.y() < other.y() { 1 } else { -1 };
            Some(Self(other.x(), self.y() + d_y))
        }
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    dir: Direction,
    size: MovementSize,
}

#[derive(Debug, PartialEq)]
enum Direction {
    L,
    R,
    U,
    D,
}

type MovementSize = u8;

fn parse(input: &str) -> Vec<Instruction> {
    let (_rest, instructions) = instructions_parser(input).unwrap();
    instructions
}

fn instructions_parser(input: &str) -> IResult<&str, Vec<Instruction>> {
    let mut parser = separated_list0(tag("\n"), instruction_parser);
    parser(input)
}

fn instruction_parser(input: &str) -> IResult<&str, Instruction> {
    let mut parser = map(separated_pair(dir_parser, tag(" "), u8), |(d, s)| {
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
}
