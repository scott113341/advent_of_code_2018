#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Point(
    pub isize,
    pub isize,
    pub isize,
    pub isize,
);

impl Point {
    pub fn parse(string: String) -> Point {
        let coords: Vec<isize> = string
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();

        Point(
            coords[0],
            coords[1],
            coords[2],
            coords[3],
        )
    }

    pub fn distance(point_1: &Point, point_2: &Point) -> isize {
        (point_1.0 - point_2.0).abs()
            + (point_1.1 - point_2.1).abs()
            + (point_1.2 - point_2.2).abs()
            + (point_1.3 - point_2.3).abs()
    }
}

type Constellation = Vec<Point>;

#[derive(Debug)]
pub struct Space {
    pub constellations: Vec<Constellation>,
}

impl Space {
    pub fn add_point(&mut self, point: &Point) {
        let mut shared_constellations: Vec<&mut Constellation> = self.constellations
            .iter_mut()
            .filter(|c| c.iter().any(|p| Point::distance(point, p) <= 3))
            .collect();

        if shared_constellations.len() == 0 {
            self.constellations.push(vec![point.clone()]);
        } else {
            let main_constellation = shared_constellations.pop().unwrap();
            main_constellation.push(point.clone());

            for c in shared_constellations {
                main_constellation.append(c);
            }

            self.constellations.retain(|c| !c.is_empty());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
    }
}
