pub fn part1(data: String) -> String {
    let cmds = cmds_parser(&data);
    dbg!(cmds.unwrap());
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
    Cd(CdPath),
    Ls(Vec<LsOutput>),
}

#[derive(Debug, PartialEq)]
enum CdPath {
    Root,
    Parent,
    Relative(String),
}

#[derive(Debug, PartialEq)]
enum LsOutput {
    DirOutput(String),
    FileOutput(String, u32),
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{not_line_ending, u32},
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult,
};

fn cmds_parser(input: &str) -> IResult<&str, Vec<Cmd>> {
    let mut parser = separated_list0(tag("\n"), cmd_parser);
    parser(input)
}

fn cmd_parser(input: &str) -> IResult<&str, Cmd> {
    let mut parser = preceded(tag("$ "), alt((cd_parser, ls_parser)));
    parser(input)
}

fn cd_path_parser(input: &str) -> IResult<&str, CdPath> {
    let root_parser = map(tag("/"), |_| CdPath::Root);
    let parent_parser = map(tag(".."), |_| CdPath::Parent);
    let relative_parser = map(not_line_ending, |name: &str| {
        CdPath::Relative(name.to_string())
    });

    let mut parser = alt((root_parser, parent_parser, relative_parser));
    parser(input)
}

fn cd_parser(input: &str) -> IResult<&str, Cmd> {
    let parser = preceded(tag("cd "), cd_path_parser);
    let mut cmd_parser = map(parser, |path| Cmd::Cd(path));

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
    let mut ls_output_parser = map(parser, |name: &str| LsOutput::DirOutput(name.to_string()));

    ls_output_parser(input)
}

fn file_parser(input: &str) -> IResult<&str, LsOutput> {
    let parser = separated_pair(u32, tag(" "), not_line_ending);
    let mut ls_output_parser = map(parser, |(size, name): (u32, &str)| {
        LsOutput::FileOutput(name.to_string(), size)
    });

    ls_output_parser(input)
}

#[cfg(test)]
mod tests {
    use super::CdPath::*;
    use super::Cmd::*;
    use super::LsOutput::*;
    use super::*;

    #[test]
    fn it_parses_cd_command() {
        let (rest, cmd) = cd_parser("cd /\nnot cd stuff").unwrap();
        assert_eq!(cmd, Cd(Root));
        assert_eq!(rest, "\nnot cd stuff");

        let (rest, cmd) = cd_parser("cd ..\nnot cd stuff").unwrap();
        assert_eq!(cmd, Cd(Parent));
        assert_eq!(rest, "\nnot cd stuff");

        let (rest, cmd) = cd_parser("cd foobar\nnot cd stuff").unwrap();
        assert_eq!(cmd, Cd(Relative("foobar".to_string())));
        assert_eq!(rest, "\nnot cd stuff");
    }

    #[test]
    fn it_parses_dir_output() {
        let (rest, ls_output_line) = dir_parser("dir foobar\nnot dir stuff").unwrap();
        assert_eq!(ls_output_line, DirOutput("foobar".to_string()));
        assert_eq!(rest, "\nnot dir stuff");
    }

    #[test]
    fn it_parses_file_output() {
        let (rest, ls_output_line) = file_parser("12345 foobar.baz\nnot file stuff").unwrap();
        assert_eq!(ls_output_line, FileOutput("foobar.baz".to_string(), 12345));
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
                DirOutput("a".to_string()),
                FileOutput("b.txt".to_string(), 14848514),
                FileOutput("c.dat".to_string(), 8504156),
                DirOutput("d".to_string())
            ])
        );
        assert_eq!(rest, "\nnot ls stuff");
    }

    #[test]
    fn it_parses_cd_or_ls_command() {
        let cd_input = "$ cd foo\n$ ls\n dir baz\n1234 file.txt\nnot command stuff";
        let (rest, command) = cmd_parser(cd_input).unwrap();
        assert_eq!(command, Cd(Relative("foo".to_string())));
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
                Cd(Root),
                Ls(vec![
                    DirOutput("a".to_string()),
                    FileOutput("b.txt".to_string(), 14848514),
                    FileOutput("c.dat".to_string(), 8504156),
                    DirOutput("d".to_string())
                ]),
                Cd(Relative("a".to_string())),
                Ls(vec![
                    DirOutput("e".to_string()),
                    FileOutput("f".to_string(), 29116),
                    FileOutput("g".to_string(), 2557),
                    FileOutput("h.lst".to_string(), 62596),
                ]),
                Cd(Relative("e".to_string())),
                Ls(vec![FileOutput("i".to_string(), 584)]),
                Cd(Parent),
                Cd(Parent),
                Cd(Relative("d".to_string())),
                Ls(vec![
                    FileOutput("j".to_string(), 4060174),
                    FileOutput("d.log".to_string(), 8033020),
                    FileOutput("d.ext".to_string(), 5626152),
                    FileOutput("k".to_string(), 7214296),
                ])
            ]
        );
        assert_eq!(rest, "".to_string());
    }
}
