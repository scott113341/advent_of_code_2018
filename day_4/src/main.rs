#![feature(dbg_macro)]

extern crate regex;

use std::collections::HashMap;

mod shift;

fn main() {
    let mut input: Vec<String> = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    input.sort();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

/// - Find guard with most minutes asleep
/// - Multiply guard id by their most frequently asleep minute
fn part_1(lines: &Vec<String>) -> usize {
    let shifts = shift::Shift::parse_all(&lines);

    // Produce a HashMap of each guard's number of minutes slept
    let mut guard_counts = HashMap::new();
    for shift in &shifts {
        let asleep_minutes = guard_counts.entry(shift.guard_id).or_insert(0);
        *asleep_minutes += shift.statuses.iter().filter(|s| s == &&shift::Status::Asleep).count();
    }
    let (most_asleep_guard_id, _) = guard_counts
        .iter()
        .max_by_key(|(_, asleep_minutes)| *asleep_minutes)
        .unwrap();

    // Find most frequently slept
    let mut minute_counts = HashMap::new();
    for shift in &shifts {
        if shift.guard_id != *most_asleep_guard_id { continue }

        let mut minute = 0;
        for status in &shift.statuses {
            if *status == shift::Status::Asleep {
                let minute_count = minute_counts.entry(minute).or_insert(0);
                *minute_count += 1;
            }
            minute += 1;
        }
    }
    let (most_asleep_minute, _) = minute_counts
        .iter()
        .max_by_key(|(_, minute_count)| *minute_count)
        .unwrap();

    most_asleep_guard_id * most_asleep_minute
}

/// - Find guard with most frequently asleep minute
/// - Multiply guard id by that minute
fn part_2(lines: &Vec<String>) -> usize {
    let shifts = shift::Shift::parse_all(&lines);
    let mut guard_minute_counts = HashMap::new();

    for shift in &shifts {
        let mut minute = 0;
        for status in &shift.statuses {
            if *status == shift::Status::Asleep {
                let id_minute = (shift.guard_id, minute);
                let count = guard_minute_counts.entry(id_minute).or_insert(0);
                *count += 1;
            }
            minute += 1;
        }
    }

    let ((guard_id, minute), _) = guard_minute_counts
        .iter()
        .max_by_key(|(_, asleep_minutes)| *asleep_minutes)
        .unwrap();

    guard_id * minute
}
