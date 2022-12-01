use aoc2022::days::day1;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use clap::Parser;

type Day = u8;
type Part = u8;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: Day,

    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Part,

    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let day = args.day;
    let part = args.part;
    let file_path = build_file_path(args.file, day);

    let data = load_data(file_path)?;
    let output = run_day_part(day, part, data);
    println!("{}", output);

    Ok(())
}

fn build_file_path(file_path_arg: Option<PathBuf>, day: Day) -> PathBuf {
    if let Some(input_file) = file_path_arg.as_deref() {
        PathBuf::from(input_file)
    } else {
        let day_file_name = format!("day{:02}.txt", day);
        ["./data", day_file_name.as_str()].iter().collect()
    }
}

fn load_data(file_path: PathBuf) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

fn run_day_part(day: Day, part: Part, data: String) -> String {
    match (day, part) {
        (1, 1) => day1::part1(data),
        (1, 2) => day1::part2(data),
        (day_m, part_m) => panic!("Day {}, part {} is not implemented", day_m, part_m),
    }
}
