pub fn part1(data: String) -> String {
    data.lines()
        .map(parse_line)
        .map(|rucksack| rucksack.find_dup())
        .map(item_priority)
        .sum::<Priority>()
        .to_string()
}

pub fn part2(data: String) -> String {
    data.lines()
        .map(parse_line)
        .collect::<Vec<Rucksack>>()
        .chunks(3)
        .map(find_dup_rucksack_item)
        .map(item_priority)
        .sum::<Priority>()
        .to_string()
}

use std::collections::HashSet;

type Item = char;
type Priority = u32;

#[derive(Debug, PartialEq)]
struct Rucksack {
    all_items: HashSet<Item>,
    compartments: Vec<HashSet<Item>>,
}

impl Rucksack {
    fn find_dup(&self) -> Item {
        *self.compartments[0]
            .intersection(&self.compartments[1])
            .next()
            .unwrap()
    }
}

fn parse_line(line: &str) -> Rucksack {
    let (contents1, contents2) = line.split_at(line.len() / 2);
    Rucksack {
        all_items: HashSet::from_iter(line.chars()),
        compartments: vec![
            HashSet::from_iter(contents1.chars()),
            HashSet::from_iter(contents2.chars()),
        ],
    }
}

fn find_dup_rucksack_item(rucksacks: &[Rucksack]) -> Item {
    *rucksacks
        .iter()
        .map(|rucksack| rucksack.all_items.clone())
        .reduce(|dups, rucksack_items| &dups & &rucksack_items)
        .unwrap()
        .iter()
        .next()
        .unwrap()
}

const LOWER_CASE_ASCII_OFFSET: u32 = 96;
const UPPER_CASE_ASCII_OFFSET: u32 = 38;

fn item_priority(item: Item) -> Priority {
    if item >= 'a' && item <= 'z' {
        (item as u32) - LOWER_CASE_ASCII_OFFSET
    } else if item >= 'A' && item <= 'Z' {
        (item as u32) - UPPER_CASE_ASCII_OFFSET
    } else {
        panic!("invalid item priority: {}", item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calcultes_part1_given_example() {
        let data = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
                    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
                    PmmdzqPrVvPwwTWBwg\n\
                    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
                    ttgJtRGJQctTZtZT\n\
                    CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = part1(data.to_string());
        assert_eq!(result, "157");
    }

    #[test]
    fn it_calcultes_part2_given_example() {
        let data = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
                    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
                    PmmdzqPrVvPwwTWBwg\n\
                    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
                    ttgJtRGJQctTZtZT\n\
                    CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = part2(data.to_string());
        assert_eq!(result, "70");
    }

    #[test]
    fn it_parses_line() {
        let line = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let compartment1 = "vJrwpWtwJgWr";
        let compartment2 = "hcsFMMfFFhFp";
        let rucksack = parse_line(line);

        assert_eq!(
            rucksack,
            Rucksack {
                all_items: HashSet::from_iter(line.chars()),
                compartments: vec![
                    HashSet::from_iter(compartment1.chars()),
                    HashSet::from_iter(compartment2.chars())
                ]
            }
        )
    }

    #[test]
    fn it_finds_common_item_in_both_rucksack_compartments() {
        let rucksack = Rucksack {
            all_items: HashSet::from_iter("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".chars()),
            compartments: vec![
                HashSet::from_iter("jqHRNqRjqzjGDLGL".chars()),
                HashSet::from_iter("rsFMfFZSrLrFZsSL".chars()),
            ],
        };
        assert_eq!(rucksack.find_dup(), 'L');
    }

    #[test]
    fn it_finds_common_item_among_multiple_rucksacks() {
        let r1 = parse_line("vJrwpWtwJgWrhcsFMMfFFhFp");
        let r2 = parse_line("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        let r3 = parse_line("PmmdzqPrVvPwwTWBwg");
        let rucksacks = [r1, r2, r3];
        assert_eq!(find_dup_rucksack_item(&rucksacks), 'r');

        let r1 = parse_line("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
        let r2 = parse_line("ttgJtRGJQctTZtZT");
        let r3 = parse_line("CrZsJsPPZsGzwwsLwLmpwMDw");
        let rucksacks = [r1, r2, r3];
        assert_eq!(find_dup_rucksack_item(&rucksacks), 'Z');
    }

    #[test]
    fn it_calculates_priority_of_item() {
        assert_eq!(item_priority('a'), 1);
        assert_eq!(item_priority('z'), 26);
        assert_eq!(item_priority('A'), 27);
        assert_eq!(item_priority('Z'), 52);
    }
}
