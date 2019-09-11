mod maze;

use maze::*;

fn main() {
    let input = include_str!("input.txt").trim().to_string();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input, 1000));
}

// What is the largest number of doors you would be required to pass through to reach a room?
fn part_1(line: &String) -> usize {
    let maze = Maze::new(line);
    maze.most_doors()
}

// How many rooms have a shortest path from your current location that pass through at least
// 1000 doors?
fn part_2(line: &String, n_doors: usize) -> usize {
    let maze = Maze::new(line);
    maze.at_least_n_doors_away(n_doors)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_1() -> String {
        "^WNE$".to_string()
    }

    fn example_2() -> String {
        "^ENWWW(NEEE|SSE(EE|N))$".to_string()
    }

    fn example_3() -> String {
        "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$".to_string()
    }

    fn example_4() -> String {
        "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$".to_string()
    }

    fn example_5() -> String {
        "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$".to_string()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&example_1()), 3);
        assert_eq!(part_1(&example_2()), 10);
        assert_eq!(part_1(&example_3()), 18);
        assert_eq!(part_1(&example_4()), 23);
        assert_eq!(part_1(&example_5()), 31);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&example_1(), 1), 3);
        assert_eq!(part_2(&example_1(), 3), 1);
    }
}
