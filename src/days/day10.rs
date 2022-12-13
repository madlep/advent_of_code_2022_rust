use nom::{
    branch::alt, bytes::complete::tag, character::complete::i32, combinator::map,
    multi::separated_list0, sequence::separated_pair, IResult,
};

pub fn part1(data: String) -> String {
    let mut signal_strength = 0_i32;
    run(&data, |cycle, x_reg| {
        if (cycle - 20) % 40 == 0 {
            let new_signal_strength = cycle * x_reg;
            signal_strength += new_signal_strength;
        }
    });
    signal_strength.to_string()
}

pub fn part2(_data: String) -> String {
    panic!("not implemented")
}

type Cycle = i32;
type Reg = i32;

fn run<F>(data: &str, mut f: F) -> ()
where
    F: FnMut(Cycle, Reg) -> (),
{
    let ops = parse(&data);
    let expanded_ops = ops.iter().flat_map(|op| match op {
        Op::Noop => vec![Op::Noop],
        ax @ Op::Addx(_) => vec![Op::Noop, *ax],
    });

    let mut x_reg = 1_i32;
    for (i, op) in expanded_ops.enumerate() {
        let cycle = i as i32 + 1;
        f(cycle, x_reg);
        match op {
            Op::Noop => (),
            Op::Addx(amount) => x_reg += amount,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Noop,
    Addx(i32),
}

fn parse(input: &str) -> Vec<Op> {
    let (_rest, ops) = ops_parser(input).unwrap();
    ops
}

fn ops_parser(input: &str) -> IResult<&str, Vec<Op>> {
    separated_list0(tag("\n"), op_parser)(input)
}

fn op_parser(input: &str) -> IResult<&str, Op> {
    alt((noop_parser, addx_parser))(input)
}

fn noop_parser(input: &str) -> IResult<&str, Op> {
    map(tag("noop"), |_| Op::Noop)(input)
}

fn addx_parser(input: &str) -> IResult<&str, Op> {
    let p = separated_pair(tag("addx"), tag(" "), i32);
    map(p, |(_, amount)| Op::Addx(amount))(input)
}
