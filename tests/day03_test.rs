use aoc2022::days::day03::{part1, part2};

const DATA: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

#[test]
fn part1_example_data() {
    assert_eq!(part1(DATA.to_string()), "157");
}

#[test]
fn part2_example_data() {
    assert_eq!(part2(DATA.to_string()), "70");
}
