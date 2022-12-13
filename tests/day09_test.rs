use aoc2022::days::day09::{part1, part2};
const DATA: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

#[test]
fn part1_example_data() {
    assert_eq!(part1(DATA.to_string()), "13");
}

#[test]
fn part2_example_data() {
    assert_eq!(part2(DATA.to_string()), "fail");
}
