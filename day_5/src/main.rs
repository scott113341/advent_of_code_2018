#![feature(dbg_macro)]

use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .to_string();

    println!("part_1: {}", part_1(input.clone()));
    println!("part_2: {}", part_2(input.clone()));
}

/// - React all adjacent opposite-polarity polymer units
/// - Return the length of the final polymer
fn part_1(polymer: String) -> usize {
    react_fully(polymer).len()
}

/// - Figure out which outcome is best if all polymer units of a type are removed
/// - Return the length of that final polymer
fn part_2(polymer: String) -> usize {
    let mut final_lengths = HashMap::new();

    let units = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i",
        "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    for unit in units.iter() {
        let modified_starting_polymer = polymer
            .to_string()
            .replace(unit, "")
            .replace(&unit.to_ascii_uppercase(), "");

        let final_length = react_fully(modified_starting_polymer).len();
        final_lengths.insert(unit, final_length);
    }

    *final_lengths.values().min().unwrap()
}

fn react(mut polymer: String) -> (bool, String) {
    let mut reacted = false;
    let mut previous_unit = ' ';

    for (idx, unit) in polymer.chars().enumerate() {
        let same_unit = unit.eq_ignore_ascii_case(&previous_unit);
        let different_polarities = !unit.eq(&previous_unit);

        if same_unit && different_polarities {
            reacted = true;
            polymer.remove(idx - 1);
            polymer.remove(idx - 1);
            break;
        } else {
            previous_unit = unit;
        }
    }

    (reacted, polymer)
}

fn react_fully(polymer: String) -> String {
    let mut reacted = true;
    let mut final_polymer = polymer;

    while reacted {
        let (new_reacted, new_final_polymer) = react(final_polymer);
        reacted = new_reacted;
        final_polymer = new_final_polymer;
    }

    final_polymer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_react() {
        assert_eq!(react("aA".to_string()), (true, "".to_string()));
        assert_eq!(react("abBA".to_string()), (true, "aA".to_string()));
        assert_eq!(react("abAB".to_string()), (false, "abAB".to_string()));
        assert_eq!(react("aabAAB".to_string()), (false, "aabAAB".to_string()));
        assert_eq!(react("dabAcCaCBAcCcaDA".to_string()), (true, "dabAaCBAcCcaDA".to_string()));
    }

    #[test]
    fn test_react_fully() {
        assert_eq!(react_fully("aA".to_string()), "".to_string());
        assert_eq!(react_fully("abBA".to_string()), "".to_string());
        assert_eq!(react_fully("abAB".to_string()), "abAB".to_string());
        assert_eq!(react_fully("aabAAB".to_string()), "aabAAB".to_string());
        assert_eq!(react_fully("dabAcCaCBAcCcaDA".to_string()), "dabCBAcaDA".to_string());
    }
}
