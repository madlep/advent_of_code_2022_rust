use aoc2022::days::day14::{part1, part2};

const DATA: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

#[test]
fn part1_example_data() {
    assert_eq!(part1(DATA.to_string()), "24");
}

#[test]
fn part2_example_data() {
    assert_eq!(part2(DATA.to_string()), "fail");
}
