#![feature(dbg_macro)]
#![feature(map_get_key_value)]

mod ground;

extern crate lazy_static;
extern crate regex;

use crate::ground::{Ground, Veins, Material::*};
use std::collections::BTreeSet;
use std::collections::HashSet;
use crate::ground::Coordinate;

fn main() {
    let veins = include_str!("input.txt")
        .trim()
        .split("\n")
        .collect();

    println!("part_1: {}", reachable_tiles(&veins));
    println!("part_2: {}", stable_tiles(&veins));
}

// Part 1
fn reachable_tiles(veins: &Veins) -> usize {
    let mut ground = Ground::parse(veins);
    let visited = run_water(&mut ground);

    visited
        .iter()
        .filter(|s| s.y >= ground.min_y && s.y <= ground.max_y)
        .count()
}

// Part 2
fn stable_tiles(veins: &Veins) -> usize {
    let mut ground = Ground::parse(veins);
    run_water(&mut ground);

    ground.coordinates
        .iter()
        .filter(|(_coord, mat)| mat == &&Water)
        .count()
}

fn run_water(ground: &mut Ground) -> BTreeSet<Coordinate> {
    let mut visited = BTreeSet::new();
    let mut sources = HashSet::new();
    sources.insert(ground.spring.clone());

    loop {
        let mut next_sources = HashSet::new();

        for source in sources.iter() {
            visited.insert(source.clone());

            let (below_coord, below_mat) = ground.below(source);

            if below_mat.is_none() {
                // Below is empty, so drip
                next_sources.insert(below_coord.clone());
                continue;
            } else {
                // Below this source is a Material:
                //   - If below is FlowingWater, then just stop
                //   - Otherwise, spread out the water
                //     - If overflow, this layer is FlowingWater and produces new source(s)
                //     - If no overflow, this layer is Water and moves the source up

                // If there is FlowingWater below this, then we can stop following this source
                // right now because it has already been explored
                if *below_mat.unwrap() == FlowingWater {
                    continue;
                }

                // This will be mutated if this source causes an overflow
                let mut overflowed = false;

                // Get leftmost legal coordinate to spread to
                let (mut left_coord, mut _left_mat) = (source.clone(), None);
                loop {
                    // Stop if this is an overflow position
                    if ground.below(&left_coord).1.is_none() {
                        next_sources.insert(left_coord.clone());
                        overflowed = true;
                        break;
                    }

                    // Stop if the left is clay
                    if let Some(next_left) = ground.left(&left_coord).1 {
                        if *next_left == Clay {
                            break;
                        }
                    }

                    // Otherwise, keep going left
                    let next_left_coord = ground.left(&left_coord);
                    left_coord = next_left_coord.0;
                    _left_mat = next_left_coord.1;
                }

                // Get rightmost legal coordinate to spread to
                let (mut right_coord, mut _right_mat) = (source.clone(), None);
                loop {
                    // Stop if this is an overflow position
                    if ground.below(&right_coord).1.is_none() {
                        next_sources.insert(right_coord.clone());
                        overflowed = true;
                        break;
                    }

                    // Stop if the right is clay
                    if let Some(next_right) = ground.right(&right_coord).1 {
                        if *next_right == Clay {
                            break;
                        }
                    }

                    // Otherwise, keep going right
                    let next_right_coord = ground.right(&right_coord);
                    right_coord = next_right_coord.0;
                    _right_mat = next_right_coord.1;
                }

                // Fill it up
                for x in left_coord.x..=right_coord.x {
                    let coord = Coordinate { y: source.y, x };
                    let water = if overflowed { FlowingWater } else { Water };

                    ground.coordinates.insert(coord.clone(), water);
                    visited.insert(coord.clone());
                }

                // If didn't overflow, then move the source up one, since we've filled this layer
                // and can think of the coordinate above this one to be our new "source"
                if !overflowed {
                    next_sources.insert(ground.above(&source).0);
                }
            }
        }

        // Prepare for next iteration through loop by filtering to only valid sources
        sources = next_sources
            .into_iter()
            .filter(|s| s.y <= ground.max_y)
            .collect();

        // Done if there are no valid sources left
        if sources.len() == 0 {
            break;
        }
    }

    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_input_1() -> Vec<&'static str> {
        vec![
            "x=495, y=2..7",
            "y=7, x=495..501",
            "x=501, y=3..7",
            "x=498, y=2..4",
            "x=506, y=1..2",
            "x=498, y=10..13",
            "x=504, y=10..13",
            "y=13, x=498..504",
        ]
    }

    // Two cups and a beam at the bottom. Water reaches the beam at different times.
    pub fn test_input_2() -> Vec<&'static str> {
        vec![
            "y=0, x=0..0",

            "y=1, x=499..499",
            "y=1, x=501..501",
            "y=2, x=499..501",

            "y=4, x=497..497",
            "y=4, x=499..499",
            "y=5, x=497..499",

            "y=7, x=496..502",
        ]
    }

    // Big cup with one inner void and one inner cup
    pub fn test_input_3() -> Vec<&'static str> {
        vec![
            "y=0, x=0..0",

            "x=494, y=2..7",
            "x=507, y=2..7",
            "y=7, x=494..507",

            "x=498, y=3..5",
            "x=501, y=3..5",
            "y=3, x=498..501",
            "y=5, x=498..501",

            "y=4, x=503..503",
            "y=4, x=505..505",
            "y=5, x=503..505",
        ]
    }

    #[test]
    fn test_reachable_tiles() {
        assert_eq!(reachable_tiles(&test_input_1()), 57);
        assert_eq!(reachable_tiles(&test_input_2()), 34);
        assert_eq!(reachable_tiles(&test_input_3()), 72);
    }

    #[test]
    fn test_stable_tiles() {
        assert_eq!(stable_tiles(&test_input_1()), 29);
    }
}
