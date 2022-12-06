use aoc2022::days::day06::{part1, part2};

const DATA_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
const DATA_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
const DATA_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
const DATA_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
const DATA_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

#[test]
fn part1_example_data() {
    assert_eq!(part1(DATA_1.to_string()), "7");
    assert_eq!(part1(DATA_2.to_string()), "5");
    assert_eq!(part1(DATA_3.to_string()), "6");
    assert_eq!(part1(DATA_4.to_string()), "10");
    assert_eq!(part1(DATA_5.to_string()), "11");
}

#[test]
fn part2_example_data() {
    assert_eq!(part2(DATA_1.to_string()), "19");
    assert_eq!(part2(DATA_2.to_string()), "23");
    assert_eq!(part2(DATA_3.to_string()), "23");
    assert_eq!(part2(DATA_4.to_string()), "29");
    assert_eq!(part2(DATA_5.to_string()), "26");
}
