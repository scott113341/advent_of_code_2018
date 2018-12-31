#![feature(dbg_macro)]

use std::char;

fn main() {
    println!("part_1: {}", find_next_n_scores(initial_kitchen(), 290431, 10));
    println!("part_2: {}", n_scores_before_run(initial_kitchen(), &vec![2,9,0,4,3,1].as_slice()));
}

fn cook(kitchen: &mut Kitchen) {
    // Calculate the current score
    let elf_1_score = kitchen.scores[kitchen.elf_1];
    let elf_2_score = kitchen.scores[kitchen.elf_2];
    let current_score = elf_1_score + elf_2_score;

    // Store the current score's digits; 0 <= current_score <= 18
    if current_score >= 10 {
        kitchen.scores.push(current_score / 10);
        kitchen.scores.push(current_score % 10);
    } else {
        kitchen.scores.push(current_score);
    }

    // Move the elves
    kitchen.elf_1 = (kitchen.elf_1 + elf_1_score + 1) % kitchen.scores.len();
    kitchen.elf_2 = (kitchen.elf_2 + elf_2_score + 1) % kitchen.scores.len();
}

fn find_next_n_scores(mut kitchen: Kitchen, after_n_recipes: usize, next_n_scores: usize) -> String {
    // Cook enough scores in the kitchen
    while kitchen.scores.len() <= (after_n_recipes + next_n_scores) {
        cook(&mut kitchen);
    }
    // Build a string of the next_n_scores
    let mut scores = String::with_capacity(next_n_scores * 2);
    for idx in after_n_recipes..(after_n_recipes + next_n_scores) {
        scores.push(char::from_digit(kitchen.scores[idx] as u32, 10).unwrap());
    }

    scores
}

fn n_scores_before_run(mut kitchen: Kitchen, pattern: &[usize]) -> usize {
    let p_len = pattern.len();

    // Keep cookin' until we find a match for the given pattern
    loop {
        cook(&mut kitchen);
        let k_len = kitchen.scores.len();

        if k_len < p_len {
            // Not enough scores to match against the pattern
            continue;
        }

        // Check the second-to-last frame [3,4,5,6,7,8,9]   7
        // Ex pattern.len() == 3                 ^^^^^
        if k_len > p_len {
            let frame = &kitchen.scores[(k_len - p_len - 1)..=(k_len - 2)];
            if frame == pattern {
                return k_len - p_len - 1;
            }
        }

        // Check the last frame [3,4,5,6,7,8,9]
        // Ex pattern.len() == 3         ^^^^^
        if kitchen.scores.len() >= pattern.len() {
            let frame = &kitchen.scores[(k_len - p_len)..=(k_len - 1)];
            if frame == pattern {
                return k_len - p_len;
            }
        }
    }
}

struct Kitchen {
    pub scores: Vec<usize>,
    pub elf_1: usize,
    pub elf_2: usize,
}

fn initial_kitchen() ->Kitchen {
    Kitchen {
        scores: vec![3, 7],
        elf_1: 0,
        elf_2: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_next_n_scores() {
        assert_eq!(find_next_n_scores(initial_kitchen(), 9, 10), "5158916779".to_string());
        assert_eq!(find_next_n_scores(initial_kitchen(), 5, 10), "0124515891".to_string());
        assert_eq!(find_next_n_scores(initial_kitchen(), 18, 10), "9251071085".to_string());
        assert_eq!(find_next_n_scores(initial_kitchen(), 2018, 10), "5941429882".to_string());
    }

    #[test]
    fn test_n_scores_before_run() {
        assert_eq!(n_scores_before_run(initial_kitchen(), &vec![5,1,5,8,9]), 9);
        assert_eq!(n_scores_before_run(initial_kitchen(), &vec![0,1,2,4,5]), 5);
        assert_eq!(n_scores_before_run(initial_kitchen(), &vec![9,2,5,1,0]), 18);
        assert_eq!(n_scores_before_run(initial_kitchen(), &vec![5,9,4,1,4]), 2018);
    }
}
