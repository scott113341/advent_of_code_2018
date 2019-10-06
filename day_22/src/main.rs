mod cave;

use crate::cave::*;
use std::collections::HashMap;

fn main() {
    let target = Coordinate { x: 9, y: 796 };
    let depth = 6969;

    println!("part_1: {}", part_1(Cave::new(&target, depth)));
    println!("part_2: {}", part_2(Cave::new(&target, depth)));
}

fn part_1(mut cave: Cave) -> usize {
    cave.compute_to(cave.target_x, cave.target_y);
    cave.regions
        .values()
        .map(|region| region.region_risk)
        .sum()
}

fn part_2(mut cave: Cave) -> usize {
    use Gear::*;

    cave.compute_to(cave.target_x, cave.target_y);

    // Places we've been, with lowest cost
    let mut visited: HashMap<CoordGear, usize> = HashMap::new();

    // Final answer
    let mut done = false;
    let target = CoordGear {
        coordinate: Coordinate { x: cave.target_x, y: cave.target_y },
        gear: Gear::Torch,
    };

    let start = CoordGear {
        coordinate: Coordinate { x: 0, y: 0 },
        gear: Torch,
    };
    visited.insert(start, 0);

    // Current edge of exploration, all lowest cost
    let mut current_nodes = vec![
        start,
    ];

    while !done {
        let mut visit_next: Vec<CoordGear> = vec![];

        for coord_gear in &current_nodes {
            let current_region = cave.regions.get(&coord_gear.coordinate).unwrap();
            let current_cost = visited.get(&coord_gear).unwrap().to_owned();

            // Valid switching
            for gear in &current_region.valid_gear() {
                let new_cost = current_cost + 7;
                let new_coord_gear = CoordGear {
                    coordinate: coord_gear.coordinate,
                    gear: *gear,
                };

                visited
                    .entry(new_coord_gear)
                    .and_modify(|c| {
                        if new_cost < *c {
                            visit_next.push(new_coord_gear);
                            *c = new_cost;
                        }
                    })
                    .or_insert_with(|| {
                        visit_next.push(new_coord_gear);
                        new_cost
                    });
            }

            for change in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (dx, dy) = change;

                if *dx < 0 && coord_gear.coordinate.x == 0 {
                    continue;
                }

                if *dy < 0 && coord_gear.coordinate.y == 0 {
                    continue;
                }

                let adjacent_coord = Coordinate {
                    x: (coord_gear.coordinate.x as isize + dx) as usize,
                    y: (coord_gear.coordinate.y as isize + dy) as usize,
                };
                let adjacent_coord_gear = CoordGear {
                    coordinate: adjacent_coord,
                    gear: coord_gear.gear,
                };

                // Expand computed cave if needed
                if !cave.regions.contains_key(&adjacent_coord) {
                    cave.compute_to(adjacent_coord.x + 100, adjacent_coord.y + 100);
                }

                let adjacent_region = cave.regions.get(&adjacent_coord).unwrap();

                // Valid moving
                if adjacent_region.valid_gear().contains(&coord_gear.gear) {
                    let new_cost = current_cost + 1;

                    visited
                        .entry(adjacent_coord_gear)
                        .and_modify(|c| {
                            if new_cost < *c {
                                visit_next.push(adjacent_coord_gear);
                                *c = new_cost;
                            }
                        })
                        .or_insert_with(|| {
                            visit_next.push(adjacent_coord_gear);
                            new_cost
                        });
                }
            }
        }

        // Check whether we're done yet: we've reached the target AND all of the next places to
        // visit already exceed the cost of reaching the target.
        done = if let Some(current_lowest_cost) = visited.get(&target) {
            visit_next.retain(|cg| visited.get(cg).unwrap() <= current_lowest_cost);
            visit_next.len() == 0
        } else {
            false
        };

        current_nodes = visit_next;
    }

    *visited.get(&target).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn example_1() -> Cave {
        Cave::new(&Coordinate { x: 10, y: 10 }, 510)
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(example_1()), 114);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(example_1()), 45);
    }
}
