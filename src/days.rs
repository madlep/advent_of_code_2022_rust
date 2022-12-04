pub mod day1;
pub mod day2;

pub type Day = u8;
pub type Part = u8;

pub fn run_day_part(day: Day, part: Part, data: String) -> String {
    match (day, part) {
        (1, 1) => day1::part1(data),
        (1, 2) => day1::part2(data),
        (2, 1) => day2::part1(data),
        (2, 2) => day2::part2(data),
        (day_m, part_m) => panic!("Day {}, part {} is not implemented", day_m, part_m),
    }
}
