use aoc2022::days::day07::{part1, part2};
const DATA: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

#[test]
fn part1_example_data() {
    assert_eq!(part1(DATA.to_string()), "95437");
}

#[test]
fn part2_example_data() {
    assert_eq!(part2(DATA.to_string()), "24933642");
}
