use aoc2022::days::day08::{part1, part2};
const DATA: &str = "\
30373
25512
65332
33549
35390";

#[test]
fn part1_example_data() {
    assert_eq!(part1(DATA.to_string()), "21");
}

#[test]
fn part2_example_data() {
    assert_eq!(part2(DATA.to_string()), "8");
}
