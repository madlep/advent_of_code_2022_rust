use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::i64,
    combinator::map,
    multi::separated_list0,
    sequence::{pair, preceded, separated_pair, tuple},
    IResult,
};

use crate::coord::{Coord, ICoord};

const ROW: i64 = 2_000_000;
const COORD_LIMITS: i64 = 4_000_000;

pub fn part1(data: String, row: Option<i64>) -> String {
    let row_num = match row {
        Some(n) => n,
        None => ROW,
    };
    let sensors = parse(&data);

    let row_coverage = RowCoverage::build_for_row(&sensors, row_num);

    row_coverage.len().to_string()
}

pub fn part2(data: String, coord_limits: Option<i64>) -> String {
    let limits = match coord_limits {
        Some(n) => n,
        None => COORD_LIMITS,
    };

    let sensors = parse(&data);

    for row_num in 0..=limits {
        let row_coverage = RowCoverage::build_for_row(&sensors, row_num);

        match row_coverage.not_covered_between(0, limits) {
            Some(xs) => {
                assert!(xs.len() == 1);
                assert!(xs[0].start == xs[0].end);
                return (xs[0].start * COORD_LIMITS + row_num).to_string();
            }
            None => (),
        }
    }
    panic!("didn't find coord");
}

#[derive(Debug)]
struct Sensor {
    coord: Coord<i64>,
    closest_beacon: Coord<i64>,
}

impl Sensor {
    fn range_at_row(&self, row: i64) -> Option<MergableRangeInclusive> {
        let md = self.coord.manhattan_distance(&self.closest_beacon);

        let md_x = md - (row - self.coord.y()).abs();

        if md_x >= 0 {
            let start = self.coord.x() - md_x;
            let end = self.coord.x() + md_x;
            Some(MergableRangeInclusive::new(start, end))
        } else {
            None
        }
    }
}

struct RowCoverage {
    row_num: i64,
    sensor_ranges: Vec<MergableRangeInclusive>,
    beacons_in_row: HashSet<i64>,
}

impl RowCoverage {
    fn new(row_num: i64) -> Self {
        Self {
            row_num,
            sensor_ranges: vec![],
            beacons_in_row: HashSet::new(),
        }
    }

    fn build_for_row(sensors: &Vec<Sensor>, row_num: i64) -> Self {
        let mut row_coverage = Self::new(row_num);
        for sensor in sensors.iter() {
            row_coverage.add_sensor(sensor);
        }

        row_coverage
    }

    fn add_sensor(&mut self, sensor: &Sensor) -> () {
        if let Some(range) = sensor.range_at_row(self.row_num) {
            self.add_range(range);
        }
        self.add_beacon(&sensor.closest_beacon);
    }

    fn add_beacon(&mut self, beacon: &Coord<i64>) -> () {
        if beacon.y() == self.row_num {
            self.beacons_in_row.insert(beacon.x());
        }
    }

    fn add_range(&mut self, new_range: MergableRangeInclusive) -> () {
        self.sensor_ranges = new_range.merge_others(&self.sensor_ranges);
        self.sensor_ranges.sort();
    }

    fn len(&self) -> i64 {
        self.sensor_ranges
            .iter()
            .map(|range| {
                let overlapping_beacons = self
                    .beacons_in_row
                    .iter()
                    .filter(|beacon_x| range.contains(**beacon_x));
                range.len() - overlapping_beacons.count() as i64
            })
            .sum::<i64>()
    }

    fn not_covered_between(&self, start: i64, end: i64) -> Option<Vec<MergableRangeInclusive>> {
        let mut not_covered = vec![];
        let mut current_x = start;

        for range in self.sensor_ranges.iter() {
            if current_x < range.start {
                not_covered.push(MergableRangeInclusive::new(current_x, range.start - 1));
            }
            current_x = (range.end + 1).clamp(0, end);
        }

        if current_x < end {
            not_covered.push(MergableRangeInclusive::new(current_x, end));
        }

        if not_covered.is_empty() {
            None
        } else {
            Some(not_covered)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MergableRangeInclusive {
    start: i64,
    end: i64,
}

impl MergableRangeInclusive {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    fn merge_others(&self, others: &Vec<Self>) -> Vec<Self> {
        let mut merged_ranges = vec![];
        let mut merged = self.clone();
        for r in others {
            match merged.merge(&r) {
                Ok(new_merged) => merged = new_merged,
                Err(NotOverlapping {}) => merged_ranges.push(r.clone()),
            }
        }
        merged_ranges.push(merged);
        merged_ranges
    }

    fn merge(&self, other: &Self) -> Result<Self, NotOverlapping> {
        if self.is_overlap(other) {
            let start = self.start.min(other.start);
            let end = self.end.max(other.end);
            Ok(Self { start, end })
        } else {
            Err(NotOverlapping {})
        }
    }

    fn is_overlap(&self, other: &Self) -> bool {
        self.contains(other.start)
            || self.contains(other.end)
            || other.contains(self.start)
            || other.contains(self.end)
    }

    fn contains(&self, v: i64) -> bool {
        self.start <= v && self.end >= v
    }

    fn len(&self) -> i64 {
        self.end - self.start + 1
    }
}

struct NotOverlapping {}

fn parse(s: &str) -> Vec<Sensor> {
    let (_rest, readings) = readings(s).unwrap();
    readings
}

fn readings(s: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list0(tag("\n"), sensor)(s)
}
// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
fn sensor(s: &str) -> IResult<&str, Sensor> {
    let p = pair(sensor_coord, beacon_coord);
    let mut p = map(p, |(sensor, beacon)| Sensor {
        coord: sensor,
        closest_beacon: beacon,
    });
    p(s)
}

//Sensor at x=2, y=18
fn sensor_coord(s: &str) -> IResult<&str, Coord<i64>> {
    preceded(tag("Sensor at "), coord)(s)
}

//: closest beacon is at x=2, y=15
fn beacon_coord(s: &str) -> IResult<&str, Coord<i64>> {
    preceded(tag(": closest beacon is at "), coord)(s)
}

//x=2, y=18
fn coord(s: &str) -> IResult<&str, Coord<i64>> {
    let p = separated_pair(coord_part("x"), tag(", "), coord_part("y"));
    let mut p = map(p, |(x, y)| Coord::new(x, y));
    p(s)
}

//x=2
fn coord_part<'a>(axis: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, i64> {
    move |s: &'a str| {
        let mut p = preceded(tuple((tag(axis), tag("="))), i64);
        p(s)
    }
}
