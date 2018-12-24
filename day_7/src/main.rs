#![feature(dbg_macro)]

mod steps;

use crate::steps::{StepInfo, Step};
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| StepInfo::parse(s.to_string()))
        .collect();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input, 5, 60));
}

fn part_1(step_infos: &Vec<StepInfo>) -> String {
    let mut steps = make_steps(&step_infos);
    let mut sequence = String::new();

    // Find the first step
    let (first_step, _step) = steps.iter().find(|(_name, step)| step.prereqs.len() == 0).unwrap();
    let mut last_step = *first_step;

    loop {
        sequence.push(last_step);
        steps.remove(&last_step);

        if let Some(next_step) = get_next_step(&sequence, &steps) {
            last_step = next_step;
        } else {
            break;
        }
    }

    sequence
}

fn part_2(step_infos: &Vec<StepInfo>, n_workers: usize, base_time: usize) -> usize {
    let mut steps = make_steps(&step_infos);
    let mut sequence = String::new();
    let mut seconds = 0;

    // Make a HashMap of workers that can be idle (None) or working until a certain time (Some)
    let mut workers: HashMap<usize, Option<(char, usize)>> = HashMap::new();
    for worker_idx in 0..n_workers {
        workers.insert(worker_idx, None);
    }

    // HashSet to store steps that are in progress
    let mut in_progress: HashSet<char> = HashSet::new();

    loop {
        // Release all workers that are done
        let done_workers = workers
            .iter_mut()
            .filter(|(_, work)| work.is_some() && work.unwrap().1 == seconds);
        for (_idx, done_worker) in done_workers {
            let finished_step = done_worker.unwrap().0;
            sequence.push(finished_step);
            in_progress.remove(&finished_step);
            steps.remove(&finished_step);
            *done_worker = None;
        }


        // Give all free workers work
        for (_idx, free_worker) in workers.iter_mut().filter(|(_idx, work)| work.is_none()) {
            if let Some(next_steps) = get_next_steps(&sequence, &steps) {
                if let Some(next_step) = next_steps.iter().find(|s| !in_progress.contains(s)) {
                    let time = seconds + base_time + (next_step.to_digit(36).unwrap() as usize) - 9;
                    *free_worker = Some((*next_step, time));
                    in_progress.insert(*next_step);
                }
            } else {
                return seconds;
            }
        }

        seconds += 1;
    }
}

fn make_steps(step_infos: &Vec<StepInfo>) -> HashMap<char, Step> {
    let mut steps = HashMap::new();

    for step_info in step_infos {
        steps
            .entry(step_info.name)
            .or_insert_with(|| Step {
                name: step_info.name,
                prereqs: vec![],
            });

        steps
            .entry(step_info.prereq_name)
            .or_insert_with(|| Step {
                name: step_info.prereq_name,
                prereqs: vec![],
            });
    }

    for step_info in step_infos {
        let step = steps.get_mut(&step_info.name).unwrap();
        step.prereqs.push(step_info.prereq_name);
    }

    steps
}

fn get_next_step(sequence: &String, steps: &HashMap<char, Step>) -> Option<char> {
    if let Some(next_steps) = get_next_steps(sequence, steps) {
        Some(*next_steps.first().unwrap())
    } else {
        None
    }
}

fn get_next_steps(sequence: &String, steps: &HashMap<char, Step>) -> Option<Vec<char>> {
    if steps.len() == 0 { return None }

    let sequence_set: HashSet<char> = HashSet::from_iter(sequence.chars());

    let mut next: Vec<char> = steps
        .iter()
        .filter_map(|(name, step)| {
            let prereq_set = HashSet::from_iter(step.prereqs.to_owned());
            if prereq_set.is_subset(&sequence_set) {
                Some(*name)
            } else {
                None
            }
        })
        .collect();

    next.sort();
    Some(next)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let steps = vec![
            StepInfo { name: 'A', prereq_name: 'C' },
            StepInfo { name: 'F', prereq_name: 'C' },
            StepInfo { name: 'B', prereq_name: 'A' },
            StepInfo { name: 'D', prereq_name: 'A' },
            StepInfo { name: 'E', prereq_name: 'B' },
            StepInfo { name: 'E', prereq_name: 'D' },
            StepInfo { name: 'E', prereq_name: 'F' },
        ];

        assert_eq!(part_1(&steps), "CABDFE".to_string());
    }

    #[test]
    fn test_part_2() {
        let steps = vec![
            StepInfo { name: 'A', prereq_name: 'C' },
            StepInfo { name: 'F', prereq_name: 'C' },
            StepInfo { name: 'B', prereq_name: 'A' },
            StepInfo { name: 'D', prereq_name: 'A' },
            StepInfo { name: 'E', prereq_name: 'B' },
            StepInfo { name: 'E', prereq_name: 'D' },
            StepInfo { name: 'E', prereq_name: 'F' },
        ];

        assert_eq!(part_2(&steps, 2, 0), 15);
    }
}
