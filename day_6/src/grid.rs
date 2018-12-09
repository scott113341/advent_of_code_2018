use std::fmt;

pub struct Grid {
    pub x_size: isize,
    pub y_size: isize,
    pub grid: Vec<Point>,
}

impl Grid {
    pub fn new(x_size: isize, y_size: isize) -> Grid {
        let mut grid = vec![];
        let total_points = x_size * y_size;

        for idx in 0..total_points {
            let point = Point {
                x: idx % x_size,
                y: idx / x_size,
                poi_distances: vec![],
            };
            grid.push(point);
        }

        Grid {
            x_size,
            y_size,
            grid,
        }
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.grid)
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
    pub poi_distances: Vec<Distance>,
}

#[derive(Clone, Debug)]
pub struct Distance {
    pub from_poi: Poi,
    pub distance: isize,
}

#[derive(Clone, Debug)]
pub struct Poi {
    pub name: String,
    pub x: isize,
    pub y: isize,
}

impl Poi {
    pub fn parse(point: &str) -> Poi {
        let mut split = point.split(", ");
        Poi {
            name: point.to_string(),
            x: split.nth(0).unwrap().parse().unwrap(),
            y: split.nth(0).unwrap().parse().unwrap(),
        }
    }
}

pub type Pois = Vec<Poi>;
