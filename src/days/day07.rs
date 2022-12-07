use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, not_line_ending, u32},
    combinator::map,
    multi::{many0, separated_list0},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

pub fn part1(_data: String) -> String {
    "foobar".to_string()
}

pub fn part2(_data: String) -> String {
    "foobar".to_string()
}

trait DirNode {
    fn size(&self) -> u32;
    fn name(&self) -> &str;
    fn parent(&self) -> &Option<&dyn DirNode>;
}

struct Dir<'a> {
    name: String,
    parent: Option<&'a dyn DirNode>,
    children: Vec<&'a dyn DirNode>,
}

impl<'a> DirNode for Dir<'a> {
    fn size(&self) -> u32 {
        self.children.iter().map(|node| node.size()).sum()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn parent(&self) -> &Option<&dyn DirNode> {
        &self.parent
    }
}

struct File<'a> {
    name: String,
    parent: Option<&'a dyn DirNode>,
    size: u32,
}

impl<'a> DirNode for File<'a> {
    fn size(&self) -> u32 {
        self.size
    }

    fn name(&self) -> &str {
        &self.name
    }
    fn parent(&self) -> &Option<&dyn DirNode> {
        &self.parent
    }
}

#[derive(Debug, PartialEq)]
enum Cmd {
    Cd(String),
    Ls(Vec<LsOutput>),
}

#[derive(Debug, PartialEq)]
enum LsOutput {
    Dir(String),
    File(u32, String),
}

fn cmds_parser(input: &str) -> IResult<&str, Vec<Cmd>> {
    let mut parser = separated_list0(tag("\n"), cmd_parser);
    parser(input)
}

fn cmd_parser(input: &str) -> IResult<&str, Cmd> {
    let mut parser = preceded(tag("$ "), alt((cd_parser, ls_parser)));
    parser(input)
}

fn cd_parser(input: &str) -> IResult<&str, Cmd> {
    let parser = preceded(tag("cd "), alt((tag(".."), tag("/"), alphanumeric1)));
    let mut cmd_parser = map(parser, |path: &str| Cmd::Cd(path.to_string()));

    cmd_parser(input)
}

fn ls_parser(input: &str) -> IResult<&str, Cmd> {
    let parser = preceded(
        tag("ls\n"),
        separated_list0(tag("\n"), alt((dir_parser, file_parser))),
    );

    let mut cmd_parser = map(parser, |outputs| Cmd::Ls(outputs));

    cmd_parser(input)
}

fn dir_parser(input: &str) -> IResult<&str, LsOutput> {
    let parser = preceded(tag("dir "), not_line_ending);
    let mut ls_output_parser = map(parser, |name: &str| LsOutput::Dir(name.to_string()));

    ls_output_parser(input)
}

fn file_parser(input: &str) -> IResult<&str, LsOutput> {
    let parser = separated_pair(u32, tag(" "), not_line_ending);
    let mut ls_output_parser = map(parser, |(size, name): (u32, &str)| {
        LsOutput::File(size, name.to_string())
    });

    ls_output_parser(input)
}

#[cfg(test)]
mod tests {
    use super::Cmd::*;
    use super::LsOutput::*;
    use super::*;

    #[test]
    fn it_parses_cd_command() {
        let (rest, cmd) = cd_parser("cd /\nnot cd stuff").unwrap();
        assert_eq!(cmd, Cd("/".to_string()));
        assert_eq!(rest, "\nnot cd stuff");
    }

    #[test]
    fn it_parses_dir_output() {
        let (rest, ls_output_line) = dir_parser("dir foobar\nnot dir stuff").unwrap();
        assert_eq!(ls_output_line, Dir("foobar".to_string()));
        assert_eq!(rest, "\nnot dir stuff");
    }

    #[test]
    fn it_parses_file_output() {
        let (rest, ls_output_line) = file_parser("12345 foobar.baz\nnot file stuff").unwrap();
        assert_eq!(ls_output_line, File(12345, "foobar.baz".to_string()));
        assert_eq!(rest, "\nnot file stuff");
    }

    #[test]
    fn it_parses_ls_command() {
        let input = "\
ls
dir a
14848514 b.txt
8504156 c.dat
dir d
not ls stuff";
        let (rest, ls) = ls_parser(input).unwrap();
        assert_eq!(
            ls,
            Ls(vec![
                Dir("a".to_string()),
                File(14848514, "b.txt".to_string()),
                File(8504156, "c.dat".to_string()),
                Dir("d".to_string())
            ])
        );
        assert_eq!(rest, "\nnot ls stuff");
    }

    #[test]
    fn it_parses_cd_or_ls_command() {
        let cd_input = "$ cd foo\n$ ls\n dir baz\n1234 file.txt\nnot command stuff";
        let (rest, command) = cmd_parser(cd_input).unwrap();
        assert_eq!(command, Cd("foo".to_string()));
        assert_eq!(
            rest,
            "\n$ ls\n dir baz\n1234 file.txt\nnot command stuff".to_string()
        );
    }

    #[test]
    fn it_parses_list_of_comands() {
        let input = "\
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
        let (rest, commands) = cmds_parser(input).unwrap();
        assert_eq!(
            commands,
            vec![
                Cd("/".to_string()),
                Ls(vec![
                    Dir("a".to_string()),
                    File(14848514, "b.txt".to_string()),
                    File(8504156, "c.dat".to_string()),
                    Dir("d".to_string())
                ]),
                Cd("a".to_string()),
                Ls(vec![
                    Dir("e".to_string()),
                    File(29116, "f".to_string()),
                    File(2557, "g".to_string()),
                    File(62596, "h.lst".to_string()),
                ]),
                Cd("e".to_string()),
                Ls(vec![File(584, "i".to_string()),]),
                Cd("..".to_string()),
                Cd("..".to_string()),
                Cd("d".to_string()),
                Ls(vec![
                    File(4060174, "j".to_string()),
                    File(8033020, "d.log".to_string()),
                    File(5626152, "d.ext".to_string()),
                    File(7214296, "k".to_string()),
                ])
            ]
        );
        assert_eq!(rest, "".to_string());
    }
}
