#![feature(dbg_macro)]

mod grid;

use crate::grid::{Coordinate, Cell};

use std::collections::HashMap;
use std::ops::RangeInclusive;

fn main() {
    let part_1_ans = part_1(7403, 3..=3);
    println!("part_1: {},{}", part_1_ans.1.x, part_1_ans.1.y);

    let part_2_ans = part_1(7403, 1..=300);
    println!("part_2: {},{},{}", part_2_ans.1.x, part_2_ans.1.y, part_2_ans.2);
}

type TotalPower = isize;
type GridSize = isize;

/// Find the 3x3 square of fuel cells with the largest total power
fn part_1(grid_sn: isize, sizes: RangeInclusive<usize>) -> (TotalPower, Coordinate, GridSize) {
    let cell_count = 300 * 300;
    let mut cells = HashMap::with_capacity(cell_count as usize);

    let mut max_power_total = None;
    let mut max_power_coordinate = None;
    let mut max_power_size = None;

    let mut cache: HashMap<(&Coordinate, GridSize), isize> = HashMap::new();

    // Populate cells
    for idx in 0..cell_count {
        let iidx = idx as isize;
        let x = iidx % 300 + 1;
        let y = iidx / 300 + 1;
        let coordinate = Coordinate { x, y };
        let cell = Cell::new(grid_sn, &coordinate);
        cells.insert(coordinate, cell);
    }

    for size in sizes {
        let size = size as isize;

        for (coord, _cell) in &cells {
            // Skip if part of the grid would go out of bounds
            if coord.x > 300 - size + 1 {
                continue;
            } else if coord.y > 300 - size + 1 {
                continue;
            }

            // Calculate the power total for this NxN square
            let mut power_total = 0;

            if let Some(smaller_power_total) = cache.get(&(coord, size - 1)) {
                // If a cached value for (N-1)x(N-1) exists, use it as a starting point, then add
                // the values on the right and bottom edges of the NxN square
                power_total = *smaller_power_total;

                for x_offset in 0..size {
                    let coordinate = Coordinate {
                        x: coord.x + x_offset,
                        y: coord.y + size - 1,
                    };
                    power_total += cells.get(&coordinate).unwrap().power_level;
                }
                for y_offset in 0..(size - 1) {
                    let coordinate = Coordinate {
                        x: coord.x + size - 1,
                        y: coord.y + y_offset,
                    };
                    power_total += cells.get(&coordinate).unwrap().power_level;
                }
            } else {
                // Perform a full calculation using each cell in the NxN square
                for x_offset in 0..size {
                    for y_offset in 0..size {
                        let coordinate = Coordinate {
                            x: coord.x + x_offset,
                            y: coord.y + y_offset,
                        };
                        power_total += cells.get(&coordinate).unwrap().power_level;
                    }
                }
            }

            // Save this power total to cache
            cache.insert((coord, size), power_total);

            // Record if this total is greater than the previous max
            if max_power_total == None || power_total > max_power_total.unwrap() {
                max_power_total = Some(power_total);
                max_power_coordinate = Some(*coord);
                max_power_size = Some(size);
            }
        }
    }

    (max_power_total.unwrap(), max_power_coordinate.unwrap(), max_power_size.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(18, 1..=3), (29, Coordinate { x: 33, y: 45 }, 3));
        assert_eq!(part_1(42, 1..=3), (30, Coordinate { x: 21, y: 61 }, 3));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_1(18, 1..=300), (113, Coordinate { x: 90, y: 269 }, 16));
    }
}
