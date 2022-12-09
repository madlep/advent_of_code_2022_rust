use std::cell::RefCell;
use std::rc::Rc;

pub fn part1(data: String) -> String {
    let cmds = parse(&data);
    let root = build_dirtree(&cmds);
    let sizes = calc_dir_sizes(root);

    sizes
        .iter()
        .filter(|size| **size <= 100_000)
        .sum::<u32>()
        .to_string()
}

pub fn part2(data: String) -> String {
    let cmds = parse(&data);
    let root = build_dirtree(&cmds);
    let sizes = calc_dir_sizes(Rc::clone(&root));

    let total_size = 70000000_u32;
    let used_size = root.borrow().calc_size();
    let remaining_size = total_size - used_size;
    let update_size = 30000000_u32;
    let required_size = update_size - remaining_size;

    dbg!(&sizes);

    sizes
        .iter()
        .filter(|size| **size > required_size)
        .min()
        .unwrap()
        .to_string()
}

fn parse(data: &str) -> Vec<Cmd> {
    let (_rest, cmds) = cmds_parser(&data).unwrap();
    cmds
}

fn build_dirtree(cmds: &Vec<Cmd>) -> Rc<RefCell<Dir>> {
    let root = Rc::new(RefCell::new(Dir::root()));
    let mut current_path = vec![Rc::clone(&root)];

    for cmd in cmds {
        {
            match cmd {
                Cmd::Cd(cd_path) => match cd_path {
                    CdPath::Root => {
                        current_path = vec![Rc::clone(&root)];
                    }
                    CdPath::Parent => {
                        current_path.pop();
                    }
                    CdPath::Relative(rel_path) => {
                        let dir = current_path.last().unwrap().borrow().cd(&rel_path).unwrap();
                        current_path.push(Rc::clone(&dir));
                    }
                },
                Cmd::Ls(outputs) => {
                    for output in outputs {
                        match output {
                            LsOutput::DirOutput(dirname) => {
                                let mut parent = current_path.last().unwrap().borrow_mut();
                                let child = Rc::new(RefCell::new(Dir::new(&dirname)));
                                parent.mk_dir(child).unwrap();
                            }
                            LsOutput::FileOutput(filename, size) => {
                                let mut parent = current_path.last().unwrap().borrow_mut();
                                parent.cp(File::new(&filename, *size)).unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
    root
}

fn calc_dir_sizes(root: Rc<RefCell<Dir>>) -> Vec<u32> {
    let sizes_cell = RefCell::new(Vec::<u32>::new());
    traverse(root, |d| {
        sizes_cell.borrow_mut().push(d.calc_size());
    });

    let mut sizes = sizes_cell.take();
    sizes.sort();
    sizes.reverse();

    sizes
}

fn traverse(dir: Rc<RefCell<Dir>>, f: impl Fn(&Dir) -> ()) -> () {
    let mut stack: Vec<Rc<RefCell<Dir>>> = vec![];

    stack.push(dir);

    while let Some(current) = stack.pop() {
        f(&current.borrow());
        for child in current.borrow().child_dirs.values() {
            stack.push(Rc::clone(child));
        }
    }
}

use std::collections::HashMap;

#[derive(Debug)]
struct NoSuchPathError(String);

#[derive(Debug)]
struct AlreadyExistsError(String);

#[derive(Debug)]
struct Dir {
    name: String,
    child_dirs: HashMap<String, Rc<RefCell<Dir>>>,
    files: HashMap<String, File>,
}

impl Dir {
    fn root() -> Self {
        Self::new("/")
    }

    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            child_dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn mk_dir(&mut self, dir: Rc<RefCell<Dir>>) -> Result<(), AlreadyExistsError> {
        let dirname = {
            let dirb = dir.borrow();
            dirb.name.clone()
        };
        if self.child_dirs.contains_key(&dirname) {
            Err(AlreadyExistsError(dirname.clone()))
        } else {
            self.child_dirs.insert(dirname.clone(), dir);
            Ok(())
        }
    }

    fn cp(&mut self, file: File) -> Result<(), AlreadyExistsError> {
        if self.files.contains_key(&file.name) {
            Err(AlreadyExistsError(file.name.clone()))
        } else {
            self.files.insert(file.name.clone(), file);
            Ok(())
        }
    }

    fn cd(&self, relative_path: &str) -> Result<Rc<RefCell<Dir>>, NoSuchPathError> {
        let child = self.child_dirs.get(relative_path);
        match child {
            Some(c) => Ok(Rc::clone(c)),
            None => Err(NoSuchPathError(relative_path.to_string())),
        }
    }

    fn calc_size(&self) -> u32 {
        let d_size: u32 = self
            .child_dirs
            .values()
            .map(|d| d.borrow().calc_size())
            .sum();

        let f_size: u32 = self.files.values().map(|f| f.size).sum();

        d_size + f_size
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

impl File {
    fn new(name: &str, size: u32) -> Self {
        Self {
            name: name.to_string(),
            size,
        }
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

fn cd_parser(input: &str) -> IResult<&str, Cmd> {
    let parser = preceded(tag("cd "), cd_path_parser);
    let mut cmd_parser = map(parser, |path| Cmd::Cd(path));

    cmd_parser(input)
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
