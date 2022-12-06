use std::collections::HashSet;

const PACKET_MARKER_SIZE: usize = 4;
const MESSAGE_MARKER_SIZE: usize = 14;

pub fn part1(data: String) -> String {
    run(data, PACKET_MARKER_SIZE).to_string()
}

pub fn part2(data: String) -> String {
    run(data, MESSAGE_MARKER_SIZE).to_string()
}

fn run(data: String, marker_size: usize) -> usize {
    for i in 0..data.len() - marker_size {
        let maybe_marker = data.get(i..i + marker_size).unwrap();
        if marker_offset(maybe_marker) {
            return i + marker_size;
        }
    }

    panic!("didn't find marker start")
}

fn marker_offset(maybe_marker: &str) -> bool {
    let mut seen_chars = HashSet::new();
    for c in maybe_marker.chars() {
        if !seen_chars.insert(c) {
            return false;
        }
    }
    true
}
