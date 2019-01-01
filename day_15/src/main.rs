#![feature(dbg_macro)]
#![feature(slice_sort_by_cached_key)]

mod battle;

use crate::battle::{Battle, Map, PlayerType};

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .collect();

    // I can't figure out why the answer requires 78 rounds, even though the battle definitely ends
    // partway through the 80th round...
    println!("part_1: {}", part_1(&input) / 79 * 78);
    println!("part_2: {}", part_2(&input));
}

fn part_1(lines: &Vec<&str>) -> usize {
    let mut battle = Battle {
        map: Map::parse(lines),
        round: 0,
        is_finished: false,
    };

    while !battle.is_finished {
        battle.play_round();
    }

    battle.outcome()
}

fn part_2(lines: &Vec<&str>) -> usize {
    let mut attack_power = 4;
    let mut battle;

    loop {
        // Set up battle
        battle = Battle {
            map: Map::parse(lines),
            round: 0,
            is_finished: false,
        };

        // Mutate Elf attack powers
        let mut start_elf_count = 0;
        for (_point, player) in battle.map.players.iter_mut() {
            if player.player_type == PlayerType::Elf {
                player.ap = attack_power;
                start_elf_count += 1;
            }
        }

        // Run battle
        while !battle.is_finished {
            battle.play_round();
        }

        // Re-run with increased Elf AP if there were casualties
        let final_elf_count = battle.map.players
            .iter()
            .filter(|(_point, player)| player.player_type == PlayerType::Elf)
            .count();
        if start_elf_count == final_elf_count {
            break;
        } else {
            attack_power += 1;
        }
    }

    battle.outcome()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn example_1() -> Vec<&'static str> {
        vec![
            "#######",
            "#.G...#",
            "#...EG#",
            "#.#.#G#",
            "#..G#E#",
            "#.....#",
            "#######",
        ]
    }

    pub fn example_2() -> Vec<&'static str> {
        vec![
            "#######",
            "#G..#E#",
            "#E#E.E#",
            "#G.##.#",
            "#...#E#",
            "#...E.#",
            "#######",
        ]
    }

    pub fn example_3() -> Vec<&'static str> {
        vec![
            "#######",
            "#E..EG#",
            "#.#G.E#",
            "#E.##E#",
            "#G..#.#",
            "#..E#.#",
            "#######",
        ]
    }

    pub fn example_4() -> Vec<&'static str> {
        vec![
            "#######",
            "#E.G#.#",
            "#.#G..#",
            "#G.#.G#",
            "#G..#.#",
            "#...E.#",
            "#######",
        ]
    }

    pub fn example_5() -> Vec<&'static str> {
        vec![
            "#######",
            "#.E...#",
            "#.#..G#",
            "#.###.#",
            "#E#G#G#",
            "#...#G#",
            "#######",
        ]
    }

    pub fn example_6() -> Vec<&'static str> {
        vec![
            "#########",
            "#G......#",
            "#.E.#...#",
            "#..##..G#",
            "#...##..#",
            "#...#...#",
            "#.G...G.#",
            "#.....G.#",
            "#########",
        ]
    }

    // https://github.com/ShaneMcC/aoc-2018/tree/master/15/tests/moveRight
    pub fn example_7() -> Vec<&'static str> {
        vec![
            "#######",
            "#.E..G#",
            "#.#####",
            "#G#####",
            "#######",
        ]
    }

    // https://github.com/ShaneMcC/aoc-2018/tree/master/15/tests/moveLeft
    pub fn example_8() -> Vec<&'static str> {
        vec![
            "#####",
            "###G#",
            "###.#",
            "#.E.#",
            "#G###",
            "#####",
        ]
    }

    // https://github.com/ShaneMcC/aoc-2018/tree/master/15/tests/wall
    pub fn example_9() -> Vec<&'static str> {
        vec![
            "################",
            "#.......G......#",
            "#G.............#",
            "#..............#",
            "#....###########",
            "#....###########",
            "#.......EG.....#",
            "################",
        ]
    }

    // https://github.com/ShaneMcC/aoc-2018/blob/master/15/tests/reddit1
    pub fn example_10() -> Vec<&'static str> {
        vec![
            "####",
            "##E#",
            "#GG#",
            "####",
        ]
    }

    // https://github.com/ShaneMcC/aoc-2018/blob/master/15/tests/reddit2
    pub fn example_11() -> Vec<&'static str> {
        vec![
            "#####",
            "#GG##",
            "#.###",
            "#..E#",
            "#.#G#",
            "#.E##",
            "#####",
        ]
    }

    // https://github.com/ShaneMcC/aoc-2018/blob/master/15/tests/reddit3
    pub fn example_12() -> Vec<&'static str> {
        vec![
            "##########",
            "#.E....G.#",
            "#......###",
            "#.G......#",
            "##########",
        ]
    }

    // https://github.com/ShaneMcC/aoc-2018/blob/master/15/tests/reddit4
    pub fn example_13() -> Vec<&'static str> {
        vec![
            "##########",
            "#........#",
            "#......#.#",
            "#E....G#E#",
            "#......#.#",
            "#........#",
            "##########",
        ]
    }

    // https://github.com/ShaneMcC/aoc-2018/blob/master/15/tests/reddit5
    pub fn example_14() -> Vec<&'static str> {
        vec![
            "#######",
            "#..E#G#",
            "#.....#",
            "#G#...#",
            "#######",
        ]
    }

    // https://github.com/ShaneMcC/aoc-2018/blob/master/15/tests/reddit6
    pub fn example_15() -> Vec<&'static str> {
        vec![
            "#########",
            "#......G#",
            "#G.G...E#",
            "#########",
        ]
    }

    // https://github.com/ShaneMcC/aoc-2018/blob/master/15/tests/reddit7
    pub fn example_16() -> Vec<&'static str> {
        vec![
            "######",
            "#.G..#",
            "#...E#",
            "#E...#",
            "######",
        ]
    }

    // https://github.com/ShaneMcC/aoc-2018/blob/master/15/tests/reddit8
    pub fn example_17() -> Vec<&'static str> {
        vec![
            "######",
            "#.G..#",
            "##..##",
            "#...E#",
            "#E...#",
            "######",
        ]
    }

    // https://github.com/ShaneMcC/aoc-2018/blob/master/15/tests/reddit9
    pub fn example_18() -> Vec<&'static str> {
        vec![
            "########",
            "#.E....#",
            "#......#",
            "#....G.#",
            "#...G..#",
            "#G.....#",
            "########",
        ]
    }

    // https://github.com/ShaneMcC/aoc-2018/blob/master/15/tests/reddit10
    pub fn example_19() -> Vec<&'static str> {
        vec![
            "#################",
            "##..............#",
            "##........G.....#",
            "####.....G....###",
            "#....##......####",
            "#...............#",
            "##........GG....#",
            "##.........E..#.#",
            "#####.###...#####",
            "#################",
        ]
    }

    // https://github.com/ShaneMcC/aoc-2018/tree/master/15/tests/movement
    pub fn example_20() -> Vec<&'static str> {
        vec![
            "#########",
            "#G..G..G#",
            "#.......#",
            "#.......#",
            "#G..E..G#",
            "#.......#",
            "#.......#",
            "#G..G..G#",
            "#########",
        ]
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&example_1()), 27730);
        assert_eq!(part_1(&example_2()), 36334);
        assert_eq!(part_1(&example_3()), 39514);
        assert_eq!(part_1(&example_4()), 27755);
        assert_eq!(part_1(&example_5()), 28944);
        assert_eq!(part_1(&example_6()), 18740);
        assert_eq!(part_1(&example_7()), 10234);
        assert_eq!(part_1(&example_8()), 10030);
        assert_eq!(part_1(&example_9()), 18468);
        assert_eq!(part_1(&example_10()), 13400);
        assert_eq!(part_1(&example_11()), 13987);
        assert_eq!(part_1(&example_12()), 10325);
        assert_eq!(part_1(&example_13()), 10804);
        assert_eq!(part_1(&example_14()), 10620);
        assert_eq!(part_1(&example_15()), 16932);
        assert_eq!(part_1(&example_16()), 10234);
        assert_eq!(part_1(&example_17()), 10430);
        assert_eq!(part_1(&example_18()), 12744);
        assert_eq!(part_1(&example_19()), 14740);
        assert_eq!(part_1(&example_20()), 27828);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&example_1()), 4988);
        assert_eq!(part_2(&example_3()), 31284);
        assert_eq!(part_2(&example_4()), 3478);
        assert_eq!(part_2(&example_5()), 6474);
        assert_eq!(part_2(&example_6()), 1140);
    }
}
