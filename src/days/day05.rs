pub fn part1(data: String) -> String {
    let mut lines = data.lines();
    let mut stacks = parse_header(&mut lines);
    run_instructions(&mut stacks, &mut lines);
    stacks.head_krates()
}

pub fn part2(_data: String) -> String {
    "foobar".to_string()
}

type Krate = char;
type StackName = char;

#[derive(Debug, PartialEq)]
struct Stacks {
    stack_names: Vec<StackName>,
    stack_map: HashMap<StackName, Vec<Krate>>,
}

impl Stacks {
    fn new(stack_names: Vec<StackName>) -> Self {
        let mut stack_map = HashMap::new();
        for name in stack_names.clone() {
            stack_map.insert(name, vec![]);
        }
        Self {
            stack_names,
            stack_map,
        }
    }

    fn add_krate(&mut self, stack_name: &StackName, krate: &Krate) {
        self.stack_map.get_mut(&stack_name).unwrap().push(*krate);
    }

    fn move_krates(&mut self, from_stack: &StackName, to_stack: &StackName, amount: usize) {
        let from = self.stack_map.get_mut(from_stack).unwrap();

        let mut popped = from.split_off(from.len() - amount);
        popped.reverse();

        self.stack_map
            .get_mut(to_stack)
            .unwrap()
            .append(&mut popped);
    }

    fn head_krates(&self) -> String {
        let mut s = String::new();
        for stack_name in &self.stack_names {
            let stack = self.stack_map.get(&stack_name).unwrap();
            s.push(*stack.last().unwrap());
        }
        s
    }
}

use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::character::complete::char;
use nom::character::complete::u32;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::IResult;

fn parse_header<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Stacks {
    let mut header_lines = lines
        .take_while(|line| !line.is_empty())
        .collect::<Vec<&str>>();
    header_lines.reverse();

    let (_rest, stack_names) = parse_stack_names(header_lines[0]).unwrap();
    let mut stacks = Stacks::new(stack_names.clone());

    for line in header_lines[1..header_lines.len()].iter() {
        let (_rest, krates) = parse_header_stack_line(line).unwrap();
        for (stack_name, krate) in stack_names.iter().zip(krates.iter()) {
            if let Some(k) = krate {
                stacks.add_krate(stack_name, k);
            }
        }
    }
    stacks
}

fn run_instructions<'a>(
    stacks: &'a mut Stacks,
    lines: &mut impl Iterator<Item = &'a str>,
) -> &'a mut Stacks {
    for line in lines {
        let (_rest, (amount, (from_stack, to_stack))) = parse_instruction(line).unwrap();
        stacks.move_krates(&from_stack, &to_stack, amount as usize);
    }
    stacks
}

fn parse_stack_names(input: &str) -> IResult<&str, Vec<StackName>> {
    let label_parser = delimited(char(' '), anychar, char(' '));
    separated_list0(char(' '), label_parser)(input)
}

fn parse_header_stack_line(input: &str) -> IResult<&str, Vec<Option<Krate>>> {
    let krate_parser = map(delimited(char('['), anychar, char(']')), |c| Some(c));
    let placeholder_parser = map(tag("   "), |_| None);
    let position_parser = alt((krate_parser, placeholder_parser));

    separated_list0(char(' '), position_parser)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, (u32, (StackName, StackName))> {
    let stacks_parser = separated_pair(anychar, tag(" to "), anychar);
    let ins_parser = separated_pair(u32, tag(" from "), stacks_parser);
    preceded(tag("move "), ins_parser)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calcultes_part1_given_example() {
        let data = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = part1(data.to_string());
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn it_parses_stack_names() {
        let labels = " 1   2   3 ";
        assert_eq!(
            parse_stack_names(labels).unwrap(),
            ("", vec!['1', '2', '3'])
        );
    }

    #[test]
    fn it_parses_header_stack_line() {
        let stack_line = "    [D]    "; // empty/stack/empty
        assert_eq!(
            parse_header_stack_line(stack_line).unwrap(),
            ("", vec![None, Some('D'), None])
        )
    }

    #[test]
    fn it_parses_header_lines() {
        let data = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";
        assert_eq!(
            parse_header(&mut data.lines()),
            Stacks {
                stack_names: vec!['1', '2', '3'],
                stack_map: HashMap::from([
                    ('1', vec!['Z', 'N']),
                    ('2', vec!['M', 'C', 'D']),
                    ('3', vec!['P'])
                ])
            }
        );
    }

    #[test]
    fn it_moves_krates() {
        let mut stacks = Stacks {
            stack_names: vec!['1', '2', '3'],
            stack_map: HashMap::from([
                ('1', vec!['Z', 'N']),
                ('2', vec!['M', 'C', 'D']),
                ('3', vec!['P']),
            ]),
        };

        stacks.move_krates(&'2', &'3', 2);
        assert_eq!(
            stacks,
            Stacks {
                stack_names: vec!['1', '2', '3'],
                stack_map: HashMap::from([
                    ('1', vec!['Z', 'N']),
                    ('2', vec!['M']),
                    ('3', vec!['P', 'D', 'C']),
                ]),
            }
        );

        stacks.move_krates(&'3', &'1', 3);
        assert_eq!(
            stacks,
            Stacks {
                stack_names: vec!['1', '2', '3'],
                stack_map: HashMap::from([
                    ('1', vec!['Z', 'N', 'C', 'D', 'P']),
                    ('2', vec!['M']),
                    ('3', vec![]),
                ]),
            }
        );
    }

    #[test]
    fn it_parses_stacks_move_instruction() {
        let ins = "move 15 from 7 to 9";
        let (rest, (mv_number, (from, to))) = parse_instruction(ins).unwrap();
        assert_eq!(rest, "");
        assert_eq!(mv_number, 15);
        assert_eq!(from, '7');
        assert_eq!(to, '9');
    }
}
