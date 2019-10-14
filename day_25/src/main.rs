#![feature(vec_remove_item)]

mod constellation;

use crate::constellation::{Point, Space};

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| Point::parse(s.to_string()))
        .collect();

    println!("part_1: {}", part_1(&input));
//    println!("part_2: {}", part_2(&input));
}



fn part_1(points: &Vec<Point>) -> usize {
    let mut space = Space { constellations: vec![] };

    for point in points {
        space.add_point(point);
    }

    space.constellations.len()
}

//fn part_2(lines: &Vec<String>) -> usize {
//    2
//}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn example_1() -> Vec<Point> {
        vec![
            Point(0, 0, 0, 0),
            Point(3, 0, 0, 0),
            Point(0, 3, 0, 0),
            Point(0, 0, 3, 0),
            Point(0, 0, 0, 3),
            Point(0, 0, 0, 6),
            Point(9, 0, 0, 0),
            Point(12, 0, 0, 0),
        ]
    }

    pub fn example_2() -> Vec<Point> {
        vec![
            Point(0, 0, 0, 0),
            Point(3, 0, 0, 0),
            Point(0, 3, 0, 0),
            Point(0, 0, 3, 0),
            Point(0, 0, 0, 3),
            Point(0, 0, 0, 6),
            Point(9, 0, 0, 0),
            Point(12, 0, 0, 0),
            Point(6, 0, 0, 0),
        ]
    }

    pub fn example_3() -> Vec<Point> {
        vec![
            Point(-1,2,2,0),
            Point(0,0,2,-2),
            Point(0,0,0,-2),
            Point(-1,2,0,0),
            Point(-2,-2,-2,2),
            Point(3,0,2,-1),
            Point(-1,3,2,2),
            Point(-1,0,-1,0),
            Point(0,2,1,-2),
            Point(3,0,0,0),
        ]
    }

    pub fn example_4() -> Vec<Point> {
        vec![
            Point(1,-1,0,1),
            Point(2,0,-1,0),
            Point(3,2,-1,0),
            Point(0,0,3,1),
            Point(0,0,-1,-1),
            Point(2,3,-2,0),
            Point(-2,2,0,0),
            Point(2,-2,0,-1),
            Point(1,-1,0,-1),
            Point(3,2,0,2),
        ]
    }

    pub fn example_5() -> Vec<Point> {
        vec![
            Point(1,-1,-1,-2),
            Point(-2,-2,0,1),
            Point(0,2,1,3),
            Point(-2,3,-2,1),
            Point(0,2,3,-2),
            Point(-1,-1,1,-2),
            Point(0,-2,-1,0),
            Point(-2,2,3,-1),
            Point(1,2,2,0),
            Point(-1,-2,0,-2),
        ]
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&example_1()), 2);
        assert_eq!(part_1(&example_2()), 1);
        assert_eq!(part_1(&example_3()), 4);
        assert_eq!(part_1(&example_4()), 3);
        assert_eq!(part_1(&example_5()), 8);
    }

    #[test]
    fn test_part_2() {
    }
}
