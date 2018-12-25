use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, PartialEq)]
pub struct Velocity {
    pub vx: isize,
    pub vy: isize,
}

#[derive(Debug, PartialEq)]
pub struct Point {
    pub initial: Coordinate,
    pub velocity: Velocity,
}

pub type Points = Vec<Point>;

impl Point {
    pub fn parse(string: &str) -> Point {
        let regex = Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();
        let caps = regex.captures(string).unwrap();

        let x = caps[1].parse().unwrap();
        let y = caps[2].parse().unwrap();
        let vx = caps[3].parse().unwrap();
        let vy = caps[4].parse().unwrap();

        Point {
            initial: Coordinate { x, y },
            velocity: Velocity { vx, vy },
        }
    }

    pub fn at_time(&self, seconds: usize) -> Coordinate {
        Coordinate {
            x: self.initial.x + self.velocity.vx * (seconds as isize),
            y: self.initial.y + self.velocity.vy * (seconds as isize),
        }
    }
}

#[derive(Debug)]
pub struct BoundingBox {
    pub min_x: isize,
    pub max_x: isize,
    pub min_y: isize,
    pub max_y: isize,
    pub width: usize,
    pub height: usize,
    pub area: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_at_time() {
        let point = Point {
            initial: Coordinate { x: 0, y: 0 },
            velocity: Velocity { vx: -1, vy: 2 },
        };
        assert_eq!(point.at_time(0), Coordinate { x: 0, y: 0 });
        assert_eq!(point.at_time(1), Coordinate { x: -1, y: 2 });
        assert_eq!(point.at_time(2), Coordinate { x: -2, y: 4 });
        assert_eq!(point.at_time(3), Coordinate { x: -3, y: 6 });
    }
}
