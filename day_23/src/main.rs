use std::convert::TryInto;

use crate::nanobot::Nanobot;
use std::collections::HashMap;

mod nanobot;

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
    for nanobot in nanobots {
        dbg!(&nanobot);
        let mut rico = HashMap::with_capacity(1_000_000_000);
        rico.insert(nanobot, 0);
    }

    0
}

fn part_2z(nanobots: &Vec<Nanobot>) -> usize {
    let scale = 10;
    let mut f = 100_000;
    let mut d = 1;

    let mut x_zmin = None;
    let mut x_zmax = None;
    let mut y_zmin = None;
    let mut y_zmax = None;
    let mut z_zmin = None;
    let mut z_zmax = None;

    let mut answer_point = None;

    loop {
        let scaled_nanobots: Vec<_> = nanobots
            .iter()
            .map(|Nanobot { x, y, z, r }| Nanobot { x: x/f, y: y/f, z: z/f, r: r/f })
            .collect();

        let x_min = x_zmin.unwrap_or_else(|| scaled_nanobots.iter().min_by_key(|n| n.x).unwrap().x);
        let x_max = x_zmax.unwrap_or_else(|| scaled_nanobots.iter().max_by_key(|n| n.x).unwrap().x);
        let y_min = y_zmin.unwrap_or_else(|| scaled_nanobots.iter().min_by_key(|n| n.y).unwrap().y);
        let y_max = y_zmax.unwrap_or_else(|| scaled_nanobots.iter().max_by_key(|n| n.y).unwrap().y);
        let z_min = z_zmin.unwrap_or_else(|| scaled_nanobots.iter().min_by_key(|n| n.z).unwrap().z);
        let z_max = z_zmax.unwrap_or_else(|| scaled_nanobots.iter().max_by_key(|n| n.z).unwrap().z);

        let check = (x_max - x_min) * (y_max - y_min) * (z_max - z_min);

        println!(
            "Checking {} points from ({} => {}), ({} => {}), ({} => {}) at f={}, d={}",
            check,
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
            f,
            d,
        );

        let mut point = Nanobot { x: 0, y: 0, z: 0, r: 0 };

        let mut highest = 1;
        let mut points_with_most_bots = Vec::with_capacity(10_000_000);

        for x in x_min..=x_max {
            point.x = x;
            for y in y_min..=y_max {
                point.y = y;
                for z in z_min..=z_max {
                    point.z = z;

                    let in_range = scaled_nanobots
                        .iter()
                        .filter(|&nb| nb.in_range(&point))
                        .count();

                    if in_range > highest {
                        println!("  New highest: {} (clearing {})", &in_range, points_with_most_bots.len());
                        highest = in_range;
                        points_with_most_bots.clear();
                    }

                    if in_range >= highest {
                        points_with_most_bots.push(point.clone());
                    }
                }
            }
        }

        let point_with_most_bots = points_with_most_bots
            .iter()
            .min_by_key(|n| n.x.abs() + n.y.abs() + n.z.abs())
            .unwrap();

        dbg!(&point_with_most_bots);

        x_zmin = Some(point_with_most_bots.x * scale - scale);
        x_zmax = Some(point_with_most_bots.x * scale + scale);
        y_zmin = Some(point_with_most_bots.y * scale - scale);
        y_zmax = Some(point_with_most_bots.y * scale + scale);
        z_zmin = Some(point_with_most_bots.z * scale - scale);
        z_zmax = Some(point_with_most_bots.z * scale + scale);

        if f == 1 {
            // If we're "unscaled", halt our search
            answer_point = Some(point_with_most_bots.clone());
            break;
        } else {
            f = f / scale;
            d = d * scale;
        }
    }

    let ap = answer_point.unwrap();
    (ap.x.abs() + ap.y.abs() + ap.z.abs()).try_into().unwrap()
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
