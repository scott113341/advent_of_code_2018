#![feature(dbg_macro)]

use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

// - Count ids containing exactly two   of any letter
// - Count ids containing exactly three of any letter
// - Return the product of these two counts
fn part_1(box_ids: &Vec<String>) -> usize {
    let mut two_count = 0;
    let mut three_count = 0;

    for box_id in box_ids {
        let counts = letter_counts(&box_id);

        if counts.values().any(|count| count == &2 ) {
            two_count += 1;
        }
        if counts.values().any(|count| count == &3 ) {
            three_count += 1;
        }
    }

    two_count * three_count
}

// - Find the two ids that only differ by one character
//   - Remove the first  character of all ids, look for a duplicate...
//   - Remove the second character of all ids, look for a duplicate...
//   - ...
// - Return the id with the differing character removed
fn part_2(box_ids: &Vec<String>) -> String {
    let mut remove_index = 0;

    loop {
        let mut modified_ids = HashSet::new();

        for box_id in box_ids {
            // Remove the remove_index-th character
            let mut mod_id = box_id.to_owned();
            mod_id.remove(remove_index);

            if modified_ids.contains(&mod_id) {
                return mod_id;
            } else {
                modified_ids.insert(mod_id);
            }
        }

        remove_index += 1;
    }
}

fn letter_counts(string: &String) -> HashMap<char, usize> {
    let mut counts = HashMap::new();

    for character in string.chars() {
        let count = counts.entry(character).or_insert(0);
        *count += 1;
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let box_ids = vec![
            "abcdef".to_string(),
            "bababc".to_string(),
            "abbcde".to_string(),
            "abcccd".to_string(),
            "aabcdd".to_string(),
            "abcdee".to_string(),
            "ababab".to_string(),
        ];
        assert_eq!(part_1(&box_ids), 12);
    }

    #[test]
    fn test_part_2() {
        let box_ids = vec![
            "abcde".to_string(),
            "fghij".to_string(),
            "klmno".to_string(),
            "pqrst".to_string(),
            "fguij".to_string(),
            "axcye".to_string(),
            "wvxyz".to_string(),
        ];
        assert_eq!(part_2(&box_ids), "fgij".to_string());
    }

    #[test]
    fn test_letter_counts() {
        let example_1 = letter_counts(&"abcdef".to_string());
        assert_eq!(example_1.len(), 6);
        assert_eq!(example_1[&'a'], 1);
        assert_eq!(example_1[&'b'], 1);
        assert_eq!(example_1[&'c'], 1);
        assert_eq!(example_1[&'d'], 1);
        assert_eq!(example_1[&'e'], 1);
        assert_eq!(example_1[&'f'], 1);

        let example_2 = letter_counts(&"bababc".to_string());
        assert_eq!(example_2.len(), 3);
        assert_eq!(example_2[&'a'], 2);
        assert_eq!(example_2[&'b'], 3);
        assert_eq!(example_2[&'c'], 1);

        let example_3 = letter_counts(&"abbcde".to_string());
        assert_eq!(example_3.len(), 5);
        assert_eq!(example_3[&'a'], 1);
        assert_eq!(example_3[&'b'], 2);
        assert_eq!(example_3[&'c'], 1);
        assert_eq!(example_3[&'d'], 1);
        assert_eq!(example_3[&'e'], 1);
    }
}
