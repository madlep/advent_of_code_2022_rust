use aoc2022::days::day04::{part1, part2};

const DATA: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

#[test]
fn part1_example_data() {
    assert_eq!(part1(DATA.to_string()), "2");
}

#[test]
fn part2_example_data() {
    assert_eq!(part2(DATA.to_string()), "4");
}
