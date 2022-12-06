use aoc2022::days::day01::{part1, part2};

const DATA: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

#[test]
fn part1_example_data() {
    assert_eq!(part1(DATA.to_string()), "24000");
}

#[test]
fn part2_example_data() {
    assert_eq!(part2(DATA.to_string()), "45000");
}
