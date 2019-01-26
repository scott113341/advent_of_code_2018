use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Ground {
    pub coordinates: BTreeMap<Coordinate, Material>,
    pub spring: Coordinate,
    pub min_y: usize,
    pub max_y: usize,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
/// Note that by declaring "y" and then "x", coordinates will be ordered by vertical position,
/// and then horizontal position. For example, { y: 2, x: 1 } < { y: 3, x: 0 }
pub struct Coordinate {
    pub y: usize,
    pub x: usize,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Material {
    Clay,
    Water,
    FlowingWater,
}

pub type Veins<'a> = Vec<&'a str>;

impl Ground {
    pub fn parse(veins: &Veins) -> Ground {
        lazy_static! {
            static ref VEIN_REGEX: Regex = Regex::new(r"(\w)=(\d+), (\w)=(\d+)\.\.(\d+)").unwrap();
        }

        let mut coordinates = BTreeMap::new();
        let spring = Coordinate { y: 0, x: 500 };

        for vein in veins {
            let cap = VEIN_REGEX.captures(&vein).unwrap();

            // Destructure the vein's regex capture into variables
            let vein_constant = &cap[1];
            let vein_constant_val = cap[2].parse().unwrap();
            let vein_range_val_start = cap[4].parse().unwrap();
            let vein_range_val_end = cap[5].parse().unwrap();

            // Loop through the vein's range piece, adding Clay coordinates
            for r in vein_range_val_start..=vein_range_val_end {
                let (y, x) = if vein_constant == "y" {
                    (vein_constant_val, r)
                } else {
                    (r, vein_constant_val)
                };
                coordinates.insert(Coordinate { y, x }, Material::Clay);
            }
        }

        let min_y = coordinates.iter().nth(0).unwrap().0.y;
        let max_y = coordinates.iter().last().unwrap().0.y;

        Ground {
            coordinates,
            spring,
            min_y,
            max_y,
        }
    }

    pub fn above(&self, coordinate: &Coordinate) -> (Coordinate, Option<&Material>) {
        let above_coord = Coordinate { y: coordinate.y - 1, x: coordinate.x };
        let above_mat = self.coordinates.get(&above_coord);
        (above_coord, above_mat)
    }

    pub fn below(&self, coordinate: &Coordinate) -> (Coordinate, Option<&Material>) {
        let below_coord = Coordinate { y: coordinate.y + 1, x: coordinate.x };
        let below_mat = self.coordinates.get(&below_coord);
        (below_coord, below_mat)
    }

    pub fn left(&self, coordinate: &Coordinate) -> (Coordinate, Option<&Material>) {
        let left_coord = Coordinate { y: coordinate.y, x: coordinate.x - 1 };
        let left_mat = self.coordinates.get(&left_coord);
        (left_coord, left_mat)
    }

    pub fn right(&self, coordinate: &Coordinate) -> (Coordinate, Option<&Material>) {
        let right_coord = Coordinate { y: coordinate.y, x: coordinate.x + 1 };
        let right_mat = self.coordinates.get(&right_coord);
        (right_coord, right_mat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ground_parse() {
        let ground = Ground::parse(&crate::tests::test_input_1());

        assert_eq!(ground.coordinates.len(), 34);
        assert!(ground.coordinates.get(&Coordinate { y: 2, x: 495 }).is_some());
        assert!(ground.coordinates.get(&Coordinate { y: 7, x: 495 }).is_some());
        assert!(ground.coordinates.get(&Coordinate { y: 7, x: 501 }).is_some());
        assert!(ground.coordinates.get(&Coordinate { y: 13, x: 498 }).is_some());
        assert!(ground.coordinates.get(&Coordinate { y: 13, x: 504 }).is_some());
    }
}
