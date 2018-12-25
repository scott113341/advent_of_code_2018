#![feature(dbg_macro)]

mod game;

use crate::game::{GameState, Marble};

use std::collections::HashMap;

fn main() {
    println!("part_1: {}", part_1(478,    71_240));
    println!("part_2: {}", part_2(478, 7_124_000));
}

/// Calculates the highest score for the marble game.  Uses a Vec, which is slow because the
/// numerous calls to "insert" and "remove" are roughly O(N).
fn part_1(player_count: usize, last_marble: usize) -> usize {
    let gs = GameState::new(player_count, last_marble);
    let mut marbles = vec![];
    let mut scores: HashMap<usize, usize> = HashMap::new();
    let mut current_marble_idx = 0;

    for (idx, next_marble) in gs.into_iter().enumerate() {
        if next_marble > 0 && next_marble % 23 == 0 {
            // Calculate player id and init score if necessary
            let player = (idx - 1) % player_count + 1;
            *scores.entry(player).or_insert(0) += next_marble;

            // Remove the marble 7 to the left
            let remove_idx = if current_marble_idx < 7 {
                marbles.len() - (7 - current_marble_idx)
            } else {
                current_marble_idx - 7
            };
            let removed_marble = marbles.remove(remove_idx);

            // Update score and current_marble_idx
            *scores.entry(player).or_insert(0) += removed_marble;
            current_marble_idx = remove_idx;
        } else {
            // Insert the marble at the appropriate index and update current_marble_idx
            let insert_idx = if marbles.len() == 0 {
                0
            } else {
                (current_marble_idx + 2) % marbles.len()
            };
            marbles.insert(insert_idx, next_marble);
            current_marble_idx = insert_idx;
        }
    }

    *scores.values().max().unwrap()
}

/// Calculates the highest score for the marble game.  Uses a linked list backed by a hash, which
/// is much faster because each insertion/removal is roughly O(C).
fn part_2(player_count: usize, last_marble: usize) -> usize {
    let gs = GameState::new(player_count, last_marble);
    let mut marbles: HashMap<usize, Marble> = HashMap::new();
    let mut scores: HashMap<usize, usize> = HashMap::new();
    let mut current_marble_id = 0;

    for (idx, next_marble) in gs.into_iter().enumerate() {
        if next_marble > 0 && next_marble % 23 == 0 {
            // Calculate player id and init score if necessary
            let player = (idx - 1) % player_count + 1;
            *scores.entry(player).or_insert(0) += next_marble;

            // Find the marble id to remove
            let mut remove_id = current_marble_id;
            for _ in 0..7 {
                remove_id = marbles.get(&remove_id).unwrap().prev;
            }

            // Remove marble and update prev/next fields
            let removed_marble = marbles.remove(&remove_id).unwrap();
            marbles.get_mut(&removed_marble.prev).unwrap().next = removed_marble.next;
            marbles.get_mut(&removed_marble.next).unwrap().prev = removed_marble.prev;

            // Update score and current_marble_id
            *scores.entry(player).or_insert(0) += removed_marble.id;
            current_marble_id = removed_marble.next;
        } else {
            // Special case for the first move
            if marbles.len() == 0 {
                marbles.insert(0, Marble { id: 0, prev: 0, next: 0 });
                continue;
            }

            // Find the two marble ids that will be to the left and right of the insertion site
            let left_marble_id = marbles.get(&current_marble_id).unwrap().next;
            let right_marble_id = marbles.get(&left_marble_id).unwrap().next;
            current_marble_id = next_marble;

            // Insert and update prev/next fields
            let new_marble = Marble {
                id: current_marble_id,
                prev: left_marble_id,
                next: right_marble_id,
            };
            marbles.insert(current_marble_id, new_marble);
            marbles.get_mut(&left_marble_id).unwrap().next = current_marble_id;
            marbles.get_mut(&right_marble_id).unwrap().prev = current_marble_id;
        }
    }

    *scores.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(9, 25), 32);
        assert_eq!(part_1(10, 1618), 8317);
        assert_eq!(part_1(13, 7999), 146373);
        assert_eq!(part_1(17, 1104), 2764);
        assert_eq!(part_1(21, 6111), 54718);
        assert_eq!(part_1(30, 5807), 37305);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(9, 25), 32);
        assert_eq!(part_2(10, 1618), 8317);
        assert_eq!(part_2(13, 7999), 146373);
        assert_eq!(part_2(17, 1104), 2764);
        assert_eq!(part_2(21, 6111), 54718);
        assert_eq!(part_2(30, 5807), 37305);
    }
}
