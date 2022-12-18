pub mod coord;
pub mod days;
use std::io::Read;
use std::path::PathBuf;

use std::fs::File;

use crate::days::*;

pub fn run(day: Day, part: Part, path: PathBuf) -> std::io::Result<String> {
    let data = load_data(path)?;
    Ok(days::run_day_part(day, part, data))
}

fn load_data(file_path: PathBuf) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}
