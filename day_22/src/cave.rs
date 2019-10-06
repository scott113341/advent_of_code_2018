use std::collections::HashMap;
use crate::cave::Gear::{Torch, Climbing, Nothing};

pub struct Cave {
    pub depth: usize,
    pub target_x: usize,
    pub target_y: usize,
    pub regions: HashMap<Coordinate, Region>,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Gear {
    Torch,
    Climbing,
    Nothing,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct CoordGear {
    pub coordinate: Coordinate,
    pub gear: Gear,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Region {
    pub erosion_level: usize,
    pub region_risk: usize,
    pub region_type: RegionType,
}

impl Region {
    pub fn new(erosion_level: usize) -> Region {
        let region_risk = erosion_level % 3;
        let region_type = match region_risk {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            2 => RegionType::Narrow,
            _ => panic!(),
        };

        Region {
            erosion_level,
            region_risk,
            region_type,
        }
    }

    pub fn valid_gear(&self) -> [Gear; 2] {
        match &self.region_type {
            RegionType::Rocky => [Torch, Climbing],
            RegionType::Wet => [Nothing, Climbing],
            RegionType::Narrow => [Nothing, Torch],
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

impl Cave {
    pub fn new(target: &Coordinate, depth: usize) -> Cave {
        Cave {
            depth: depth,
            target_x: target.x,
            target_y: target.y,
            regions: HashMap::new(),
        }
    }

    pub fn compute_to(&mut self, to_x: usize, to_y: usize) {
        for y in 0..=to_y {
            for x in 0..=to_x {
                let coord = Coordinate { x, y };
                if self.regions.contains_key(&coord) {
                    continue;
                }

                let geologic_index = self.geologic_index(&coord);
                let erosion_level = self.erosion_level(geologic_index);
                let region = Region::new(erosion_level);

                self.regions.insert(coord, region);
            }
        }
    }

    pub fn erosion_level(&self, geologic_index: usize) -> usize {
        (geologic_index + self.depth) % 20183
    }

    pub fn geologic_index(&self, coordinate: &Coordinate) -> usize {
        let Cave { target_x: tx, target_y: ty, .. } = self;

        match coordinate {
            Coordinate { x: 0, y: 0 } => 0,
            Coordinate { x, y } if (x == tx && y == ty) => 0,
            Coordinate { x, y: 0 } => x * 16807,
            Coordinate { x: 0, y } => y * 48271,
            Coordinate { x, y } => {
                let coord_left = self.regions.get(&Coordinate { x: x - 1, y: *y }).unwrap();
                let coord_up = self.regions.get(&Coordinate { x: *x, y: y - 1 }).unwrap();
                coord_left.erosion_level * coord_up.erosion_level
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::tests::example_1;

    #[test]
    fn test_cave() {
        let mut cave = example_1();
        cave.compute_to(cave.target_x, cave.target_y);

        assert_eq!(cave.geologic_index(&Coordinate { x: 0, y: 0 }), 0);
        assert_eq!(cave.geologic_index(&Coordinate { x: 1, y: 0 }), 16807);
        assert_eq!(cave.geologic_index(&Coordinate { x: 0, y: 1 }), 48271);
        assert_eq!(cave.geologic_index(&Coordinate { x: 1, y: 1 }), 145722555);
        assert_eq!(cave.geologic_index(&Coordinate { x: 10, y: 10 }), 0);

        assert_eq!(*cave.regions.get(&Coordinate { x: 0, y: 0 }).unwrap(), Region { erosion_level: 510, region_type: RegionType::Rocky, region_risk: 0 });
        assert_eq!(*cave.regions.get(&Coordinate { x: 1, y: 0 }).unwrap(), Region { erosion_level: 17317, region_type: RegionType::Wet, region_risk: 1 });
        assert_eq!(*cave.regions.get(&Coordinate { x: 0, y: 1 }).unwrap(), Region { erosion_level: 8415, region_type: RegionType::Rocky, region_risk: 0 });
        assert_eq!(*cave.regions.get(&Coordinate { x: 1, y: 1 }).unwrap(), Region { erosion_level: 1805, region_type: RegionType::Narrow, region_risk: 2 });
        assert_eq!(*cave.regions.get(&Coordinate { x: 10, y: 10 }).unwrap(), Region { erosion_level: 510, region_type: RegionType::Rocky, region_risk: 0 });
    }
}
