#![feature(dbg_macro)]

mod point;

extern crate regex;

use crate::point::{Point, Points, BoundingBox, Coordinate};

fn main() {
    let points = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| Point::parse(s))
        .collect();

    find_and_print_message(&points);
}

/// Find the minimum bounding box that fits all points in the first 100,000 "frames", and print
/// out the state for manual inspection!
fn find_and_print_message(points: &Points) {
    let (_min_bounding, time) = min_bounding(points, 100_000).unwrap();

    // Print the few seconds around the minimum bounding box
    for t in (time - 2)..(time + 3) {
        let coords_at_t: Vec<Coordinate> = points.iter().map(|point| point.at_time(t)).collect();
        let bounding = bounding_for(&coords_at_t);

        println!("\n\n-- At {} seconds", t);
        println!("-- {} x {} ({} pixels)", bounding.width, bounding.height, bounding.area);
        print_coords(&coords_at_t, &bounding);
    }
}

fn bounding_for(coords: &Vec<Coordinate>) -> BoundingBox {
    let min_x = coords.iter().min_by_key(|c| c.x).unwrap().x;
    let max_x = coords.iter().max_by_key(|c| c.x).unwrap().x;
    let min_y = coords.iter().min_by_key(|c| c.y).unwrap().y;
    let max_y = coords.iter().max_by_key(|c| c.y).unwrap().y;

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let area = (width * height) as usize;

    BoundingBox {
        min_x,
        max_x,
        min_y,
        max_y,
        width,
        height,
        area,
    }
}

/// Returns a tuple of the minimum-area BoundingBox + time for for the period from 0..max_seconds
fn min_bounding(points: &Points, max_seconds: usize) -> Option<(BoundingBox, usize)> {
    let mut smallest_box = None;

    for time in 0..(max_seconds + 1) {
        let coords_at_t: Vec<Coordinate> = points
            .iter()
            .map(|point| point.at_time(time))
            .collect();
        let this_box = bounding_for(&coords_at_t);

        match &smallest_box {
            None => smallest_box = Some((this_box, time)),
            Some(largest) => {
                if this_box.area < largest.0.area {
                    smallest_box = Some((this_box, time));
                }
            },
        }
    }

    smallest_box
}

fn print_coords(coords: &Vec<Coordinate>, bounding_box: &BoundingBox) {
    // Create a 2-dimensional grid
    let mut rows = vec![];
    for _ in 0..bounding_box.height {
        rows.push(vec![false; bounding_box.width]);
    }

    // Mark all coordinates as true
    for coord in coords {
        let x = coord.x - bounding_box.min_x;
        let y = coord.y - bounding_box.min_y;
        rows[y as usize][x as usize] = true;
    }

    // Print
    for row in rows {
        for pixel in row {
            if pixel {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Points {
        let lines = vec![
            "position=< 9,  1> velocity=< 0,  2>",
            "position=< 7,  0> velocity=<-1,  0>",
            "position=< 3, -2> velocity=<-1,  1>",
            "position=< 6, 10> velocity=<-2, -1>",
            "position=< 2, -4> velocity=< 2,  2>",
            "position=<-6, 10> velocity=< 2, -2>",
            "position=< 1,  8> velocity=< 1, -1>",
            "position=< 1,  7> velocity=< 1,  0>",
            "position=<-3, 11> velocity=< 1, -2>",
            "position=< 7,  6> velocity=<-1, -1>",
            "position=<-2,  3> velocity=< 1,  0>",
            "position=<-4,  3> velocity=< 2,  0>",
            "position=<10, -3> velocity=<-1,  1>",
            "position=< 5, 11> velocity=< 1, -2>",
            "position=< 4,  7> velocity=< 0, -1>",
            "position=< 8, -2> velocity=< 0,  1>",
            "position=<15,  0> velocity=<-2,  0>",
            "position=< 1,  6> velocity=< 1,  0>",
            "position=< 8,  9> velocity=< 0, -1>",
            "position=< 3,  3> velocity=<-1,  1>",
            "position=< 0,  5> velocity=< 0, -1>",
            "position=<-2,  2> velocity=< 2,  0>",
            "position=< 5, -2> velocity=< 1,  2>",
            "position=< 1,  4> velocity=< 2,  1>",
            "position=<-2,  7> velocity=< 2, -2>",
            "position=< 3,  6> velocity=<-1, -1>",
            "position=< 5,  0> velocity=< 1,  0>",
            "position=<-6,  0> velocity=< 2,  0>",
            "position=< 5,  9> velocity=< 1, -2>",
            "position=<14,  7> velocity=<-2,  0>",
            "position=<-3,  6> velocity=< 2, -1>",
        ];
        lines
            .iter()
            .map(|line| Point::parse(line))
            .collect()
    }

    #[test]
    fn test_min_bounding() {
        let (smallest, time) = min_bounding(&test_input(), 10).unwrap();
        assert_eq!(smallest.width, 10);
        assert_eq!(smallest.height, 8);
        assert_eq!(smallest.area, 80);
        assert_eq!(time, 3);
    }
}
