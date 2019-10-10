mod nanobot;

use crate::nanobot::Nanobot;

fn main() {
    let nanobots = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| Nanobot::parse(&s.to_string()))
        .collect();

    println!("part_1: {}", part_1(&nanobots));
    println!("part_2: {}", part_2(&nanobots));
}

fn part_1(nanobots: &Vec<Nanobot>) -> usize {
    let strongest_nanobot = nanobots
        .iter()
        .max_by_key(|nb| nb.r)
        .unwrap();

    nanobots
        .iter()
        .filter(|&nb| strongest_nanobot.in_range(nb))
        .count()
}

fn part_2(nanobots: &Vec<Nanobot>) -> usize {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![
            "pos=<0,0,0>, r=4".to_string(),
            "pos=<1,0,0>, r=1".to_string(),
            "pos=<4,0,0>, r=3".to_string(),
            "pos=<0,2,0>, r=1".to_string(),
            "pos=<0,5,0>, r=3".to_string(),
            "pos=<0,0,3>, r=1".to_string(),
            "pos=<1,1,1>, r=1".to_string(),
            "pos=<1,1,2>, r=1".to_string(),
            "pos=<1,3,1>, r=1".to_string(),
        ];
        let nanobots = input
            .iter()
            .map(|s| Nanobot::parse(&s.to_string()))
            .collect();

        assert_eq!(part_1(&nanobots), 7);
    }
}
