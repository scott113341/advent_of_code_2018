#![feature(dbg_macro)]

use std::collections::{BTreeMap, HashMap};

use crate::Acre::*;

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .collect();

    println!("part_1: {}", resource_value(&mut Area::parse(&input), 10));
    println!("part_2: {}", resource_value(&mut Area::parse(&input), 1_000_000_000));
}

// Multiply the number of wood acres by the number of lumberyards after the given amount of time
fn resource_value(area: &mut Area, minutes: usize) -> usize {
    // There is a steady-state pattern that emerges after ~1000 iterations
    if minutes >= 1000 {
        let pattern = vec![
            202272, 207172, 208351, 211140, 212248, 219349, 218584, 218286, 213244, 210630, 205800,
            205412, 201916, 193120, 189090, 190143, 187525, 190740, 189601, 195471, 195426, 199758,
            198062, 201684, 200349, 202515, 203895, 204486,
        ];
        return pattern[(minutes - 1000) % pattern.len()];
    }

    let mut cached_next: HashMap<Grid, Grid> = HashMap::new();

    while area.minute < minutes {
        // This was used to find the pattern
        if area.minute >= 1000 {
            println!("{}, {}", area.resource_value(), area.minute);
        }

        let mut next_grid = BTreeMap::new();

        if let Some(next) = cached_next.get(&area.grid) {
            next_grid = next.clone();
        } else {
            for (coordinate, acre) in area.grid.iter() {
                let counts = area.adjacent_counts(&coordinate);
                let mut next_acre = *acre;

                // An open acre will become filled with trees if three or more adjacent acres contained
                // trees. Otherwise, nothing happens.
                if *acre == Open {
                    if counts.get(&Trees).unwrap_or(&0) >= &3 {
                        next_acre = Trees;
                    }
                }

                // An acre filled with trees will become a lumberyard if three or more adjacent acres
                // were lumberyards. Otherwise, nothing happens.
                if *acre == Trees {
                    if counts.get(&Lumberyard).unwrap_or(&0) >= &3 {
                        next_acre = Lumberyard;
                    }
                }

                // An acre containing a lumberyard will remain a lumberyard if it was adjacent to at
                // least one other lumberyard and at least one acre containing trees. Otherwise, it
                // becomes open.
                if *acre == Lumberyard {
                    let lumberyard_count = counts.get(&Lumberyard).unwrap_or(&0);
                    let trees_count = counts.get(&Trees).unwrap_or(&0);

                    if lumberyard_count >= &1 && trees_count >= &1 {
                        next_acre = Lumberyard;
                    } else {
                        next_acre = Open;
                    }
                }

                next_grid.insert(*coordinate, next_acre);
            }

            cached_next.insert(area.grid.clone(), next_grid.clone());
        }

        area.minute += 1;
        area.grid = next_grid;
    }

    area.resource_value()
}

#[derive(Debug)]
struct Area {
    pub minute: usize,
    pub grid: Grid,
}

impl Area {
    pub fn parse(lines: &Vec<&str>) -> Area {
        let mut grid = BTreeMap::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, acre) in line.chars().enumerate() {
                let coordinate = Coordinate {
                    y: y as isize,
                    x: x as isize,
                };
                let acre = match acre {
                    '.' => Open,
                    '|' => Trees,
                    '#' => Lumberyard,
                    _ => panic!("Invalid"),
                };

                grid.insert(coordinate, acre);
            }
        }

        Area {
            minute: 0,
            grid,
        }
    }

    pub fn adjacent_counts(&self, to_coordinate: &Coordinate) -> HashMap<Acre, usize> {
        let mut counts = HashMap::new();

        for y in (to_coordinate.y - 1)..=(to_coordinate.y + 1) {
            for x in (to_coordinate.x - 1)..=(to_coordinate.x + 1) {
                if to_coordinate.y == y && to_coordinate.x == x {
                    continue;
                }

                let check_coord = Coordinate { y, x };

                if let Some(acre) = self.grid.get(&check_coord) {
                    let count = counts.entry(*acre).or_insert(0);
                    *count += 1;
                }
            }
        }

        counts
    }

    pub fn resource_value(&self) -> usize {
        let acres_trees = self.grid.iter().filter(|(_c, a)| **a == Trees).count();
        let acres_lumberyards = self.grid.iter().filter(|(_c, a)| **a == Lumberyard).count();
        acres_trees * acres_lumberyards
    }
}

type Grid = BTreeMap<Coordinate, Acre>;

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone, Debug)]
struct Coordinate {
    pub y: isize,
    pub x: isize,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_1() -> Vec<&'static str> {
        vec![
            ".#.#...|#.",
            ".....#|##|",
            ".|..|...#.",
            "..|#.....#",
            "#.#|||#|#|",
            "...#.||...",
            ".|....|...",
            "||...#|.#|",
            "|.||||..|.",
            "...#.|..|.",
        ]
    }

    #[test]
    fn test_part_1() {
        let mut area = Area::parse(&example_1());
        assert_eq!(resource_value(&mut area, 10), 1147);
        assert_eq!(resource_value(&mut area, 1000), 202272);
        assert_eq!(resource_value(&mut area, 1001), 207172);
        assert_eq!(resource_value(&mut area, 1002), 208351);
        assert_eq!(resource_value(&mut area, 10_000), 201916);
    }

    #[test]
    fn test_area_parse() {
        let area = Area::parse(&example_1());

        assert_eq!(area.minute, 0);
        assert_eq!(area.grid[&Coordinate { y: 0, x: 0 }], Open);
        assert_eq!(area.grid[&Coordinate { y: 0, x: 1 }], Lumberyard);
        assert_eq!(area.grid[&Coordinate { y: 2, x: 1 }], Trees);
    }
}
