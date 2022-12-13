use nom::{
    branch::alt, bytes::complete::tag, character::complete::i32, combinator::map,
    multi::separated_list0, sequence::separated_pair, IResult,
};

pub fn part1(data: String) -> String {
    let mut signal_strength = 0_i32;
    run(&data, |i, x_reg| {
        let cycle = i as i32 + 1;
        if (cycle - 20) % 40 == 0 {
            let new_signal_strength = cycle * x_reg;
            signal_strength += new_signal_strength;
        }
    });
    signal_strength.to_string()
}

const CRT_WIDTH: usize = 40;
pub fn part2(data: String) -> String {
    let mut crt = String::new();
    run(&data, |i, x_reg| {
        let pixel = i % CRT_WIDTH;
        let pixel_output = if (x_reg - 1..=x_reg + 1).contains(&(pixel as i32)) {
            '#'
        } else {
            '.'
        };
        crt.push(pixel_output);
        if pixel == CRT_WIDTH - 1 {
            crt.push('\n');
        }
    });
    crt
}

type Idx = usize;
type Reg = i32;

fn run<F>(data: &str, mut f: F) -> ()
where
    F: FnMut(Idx, Reg) -> (),
{
    let ops = parse(&data);
    let expanded_ops = ops.iter().flat_map(|op| match op {
        Op::Noop => vec![Op::Noop],
        ax @ Op::Addx(_) => vec![Op::Noop, *ax],
    });

    let mut x_reg = 1_i32;
    for (i, op) in expanded_ops.enumerate() {
        f(i, x_reg);
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
