use aoc2022::days::day05::{part1, part2};

const DATA: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[test]
fn part1_example_data() {
    assert_eq!(part1(DATA.to_string()), "CMZ");
}

#[test]
fn part2_example_data() {
    assert_eq!(part2(DATA.to_string()), "MCD");
}
