use aoc2022::days::day12::{part1, part2};

const DATA: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

#[test]
fn part1_example_data() {
    assert_eq!(part1(DATA.to_string()), "31");
}

#[test]
fn part2_example_data() {
    assert_eq!(part2(DATA.to_string()), "fail");
}
