pub fn part1(data: String) -> String {
    data.lines()
        .map(parse_line)
        .filter(is_full_overlap)
        .count()
        .to_string()
}

pub fn part2(data: String) -> String {
    data.lines()
        .map(parse_line)
        .filter(is_partial_overlap)
        .count()
        .to_string()
}

use lazy_static::lazy_static;
use regex::Regex;

use std::ops::RangeInclusive;

type Section = u32;

fn parse_line(line: &str) -> (RangeInclusive<Section>, RangeInclusive<Section>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    let s1_start = &caps[1].parse::<Section>().unwrap();
    let s1_end = &caps[2].parse::<Section>().unwrap();
    let s2_start = &caps[3].parse::<Section>().unwrap();
    let s2_end = &caps[4].parse::<Section>().unwrap();

    let s1 = *s1_start..=*s1_end;
    let s2 = *s2_start..=*s2_end;

    (s1, s2)
}

fn is_full_overlap(ranges: &(RangeInclusive<Section>, RangeInclusive<Section>)) -> bool {
    let (r1, r2) = ranges;
    let (r1_s, r1_e) = r1.clone().into_inner();
    let (r2_s, r2_e) = r2.clone().into_inner();

    (r1_s <= r2_s && r1_e >= r2_e) || (r2_s <= r1_s && r2_e >= r1_e)
}

fn is_partial_overlap(ranges: &(RangeInclusive<Section>, RangeInclusive<Section>)) -> bool {
    let (r1, r2) = ranges;
    let (r1_s, r1_e) = r1.clone().into_inner();
    let (r2_s, r2_e) = r2.clone().into_inner();

    r1.contains(&r2_s) || r1.contains(&r2_e) || r2.contains(&r1_s) || r2.contains(&r1_e)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_input_line() {
        assert_eq!(parse_line(&"2-4,6-8"), (2..=4, 6..=8))
    }
}
