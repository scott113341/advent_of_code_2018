#![feature(dbg_macro)]

mod types;

use crate::types::*;

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .collect();

    println!("part_1: {}", pot_sum(&input, 20));

    // 50 billion generations turned out to be too much, even after making some optimizations to
    // #add_or_remove_empty_start_end_pots.  After examining the state after thousands of
    // generations, it became clear that the evolution becomes stable, and the sum of the pots can
    // be described by: 52g + 1872 for large g.  To see for yourself, un-comment the code in
    // #simulate.
    let part_2: usize = 52 * 50_000_000_000 + 1872;
    println!("part_2: {}", part_2);
}

/// Returns sum of the numbers of all pots which contain a plant
fn pot_sum(input: &Vec<&str>, generations: usize) -> isize {
    let mut pot_sim = parse_pot_sim(input);
    simulate(&mut pot_sim, generations);

    pot_sim.pots
        .iter()
        .map(|(pot_id, pot)| if pot.has_plant { *pot_id } else { 0 })
        .sum()
}

// What is the sum of the numbers of all pots which contain a plant
fn simulate(pot_sim: &mut PotSim, to_generation: usize) {
    while pot_sim.generation < to_generation {
        // if pot_sim.generation % 100_000 == 0 {
        //     let sum: isize = pot_sim.pots
        //         .iter()
        //         .map(|(pot_id, pot)| if pot.has_plant { *pot_id } else { 0 })
        //         .sum();
        //     println!("Generation {} at with {} pots: {}", pot_sim.generation, pot_sim.pots.len(), sum);
        // }
        pot_sim.next_generation();
    }
}

fn parse_pot_sim(input: &Vec<&str>) -> PotSim {
    let pots = input[0][15..]
        .chars()
        .enumerate()
        .map(|(pot_id, state)| {
            let pot = Pot { has_plant: state == '#' };
            (pot_id as isize, pot)
        })
        .collect();

    let rules = input[2..]
        .iter()
        .map(|rule_line| {
            let r: Vec<bool> = rule_line[0..=4]
                .chars()
                .map(|state| state == '#')
                .collect();
            let rule = Rule(r[0], r[1], r[2], r[3], r[4]);
            let yields_plant = rule_line[9..=9] == *"#";

            (rule, yields_plant)
        })
        .collect();

    PotSim {
        generation: 0,
        pots,
        rules,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Vec<&'static str> {
        vec![
            "initial state: #..#.#..##......###...###",
            "",
            "...## => #",
            "..#.. => #",
            ".#... => #",
            ".#.#. => #",
            ".#.## => #",
            ".##.. => #",
            ".#### => #",
            "#.#.# => #",
            "#.### => #",
            "##.#. => #",
            "##.## => #",
            "###.. => #",
            "###.# => #",
            "####. => #",
        ]
    }

    #[test]
    fn test_parse_pot_sim() {
        let pot_sim = parse_pot_sim(&test_input());

        assert_eq!(pot_sim.generation, 0);

        assert_eq!(pot_sim.pots.len(), 25);
        assert_eq!(pot_sim.pots.get(&0).unwrap().has_plant, true);
        assert_eq!(pot_sim.pots.get(&1).unwrap().has_plant, false);
        assert_eq!(pot_sim.pots.get(&2).unwrap().has_plant, false);
        assert_eq!(pot_sim.pots.get(&3).unwrap().has_plant, true);

        assert_eq!(pot_sim.rules.len(), 14);
        assert_eq!(*pot_sim.rules.get(&Rule(false, false, false, true, true)).unwrap(), true);
        assert_eq!(*pot_sim.rules.get(&Rule(false, false, true, false, false)).unwrap(), true);
    }

    #[test]
    fn test_simulate() {
        let mut pot_sim_3 = parse_pot_sim(&test_input());
        simulate(&mut pot_sim_3, 3);
        assert_eq!(pot_sim_3.pots.get(&-1).unwrap().has_plant, true);
        assert_eq!(pot_sim_3.pots.get(&0).unwrap().has_plant, false);
        assert_eq!(pot_sim_3.pots.get(&1).unwrap().has_plant, true);

        let mut pot_sim_20 = parse_pot_sim(&test_input());
        simulate(&mut pot_sim_20, 20);
        assert_eq!(pot_sim_20.pots.get(&-1).unwrap().has_plant, false);
        assert_eq!(pot_sim_20.pots.get(&0).unwrap().has_plant, false);
        assert_eq!(pot_sim_20.pots.get(&1).unwrap().has_plant, false);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(pot_sum(&test_input(), 20), 325);
    }
}
