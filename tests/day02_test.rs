use aoc2022::days::day02::{part1, part2};
const DATA: &str = "\
A Y 
B X 
C Z ";

#[test]
fn part1_example_data() {
    assert_eq!(part1(DATA.to_string()), "15");
}

#[test]
fn part2_example_data() {
    assert_eq!(part2(DATA.to_string()), "12");
}
