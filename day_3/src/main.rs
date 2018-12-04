#![feature(dbg_macro)]

extern crate regex;

mod claim;

use std::collections::HashMap;
use std::collections::hash_map::Entry::Occupied;
use self::claim::Claim;

type Claims = Vec<Claim>;
type FabricCoordinate = (usize, usize);
type Fabric = HashMap<FabricCoordinate, Claims>;

fn main() {
    let claims = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| Claim::parse(&s.to_string()))
        .collect();

    println!("part_1: {}", part_1(&claims));
    println!("part_2: {}", part_2(&claims));
}

fn build_fabric(claims: &Claims) -> Fabric {
    let mut fabric: Fabric = HashMap::new();

    // Go through each claim
    for claim in claims {
        // For each column
        for x in claim.left..(claim.left + claim.width) {
            // For each row
            for y in claim.top..(claim.top + claim.height) {
                let coordinate = (x, y);
                let claims = fabric.entry(coordinate).or_insert(Claims::new());
                claims.push((*claim).to_owned());
            }
        }
    }

    fabric
}

// How many square inches of fabric are within two or more claims?
fn part_1(claims: &Claims) -> usize {
    build_fabric(claims)
        .values()
        .filter(|sq_claims| sq_claims.len().ge(&&2))
        .count()
}

// What's the id of the Claim that doesn't overlap?
fn part_2(claims: &Claims) -> usize {
    let mut fabric = build_fabric(&claims);

    // Go through each claim and see if it has no overlap
    for claim in claims {
        let mut no_overlap = true;

        // For each column
        for x in claim.left..(claim.left + claim.width) {
            // For each row
            for y in claim.top..(claim.top + claim.height) {
                let coordinate = (x, y);

                if let Occupied(claims) = fabric.entry(coordinate) {
                    if claims.get().len().gt(&&1) {
                        no_overlap = false;
                    }
                }
            }
        }

        if no_overlap {
            return claim.id;
        }
    }

    panic!("No Claim has no overlaps");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let lines = vec![
            Claim::parse(&"#1 @ 1,3: 4x4".to_string()),
            Claim::parse(&"#2 @ 3,1: 4x4".to_string()),
            Claim::parse(&"#3 @ 5,5: 2x2".to_string()),
        ];
        assert_eq!(part_1(&lines), 4);
    }

    #[test]
    fn test_part_2() {
        let lines = vec![
            Claim::parse(&"#1 @ 1,3: 4x4".to_string()),
            Claim::parse(&"#2 @ 3,1: 4x4".to_string()),
            Claim::parse(&"#3 @ 5,5: 2x2".to_string()),
        ];
        assert_eq!(part_2(&lines), 3);
    }
}
