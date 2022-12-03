pub mod days;
use std::io::Read;
use std::path::PathBuf;

use std::fs::File;

use crate::days::*;

pub fn run(day: Day, part: Part, path: PathBuf) -> std::io::Result<String> {
    let data = load_data(path)?;
    Ok(run_day_part(day, part, data))
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
