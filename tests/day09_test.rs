use aoc2022::days::day09::{part1, part2};

#[test]
fn part1_example_data() {
    let data = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(part1(data.to_string()), "13");
}

#[test]
fn part2_example_data() {
    let data = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
    assert_eq!(part2(data.to_string()), "36");
}
