#[derive(Debug, PartialEq)]
pub struct Cell {
    pub power_level: isize,
}

impl Cell {
    pub fn new(grid_sn: isize, coordinate: &Coordinate) -> Cell {
        // Find the fuel cell's rack ID, which is its X coordinate plus 10.
        let rack_id = coordinate.x + 10;

        // Begin with a power level of the rack ID times the Y coordinate.
        let mut power_level = rack_id * coordinate.y;

        // Increase the power level by the value of the grid serial number (your puzzle input).
        power_level += grid_sn;

        // Set the power level to itself multiplied by the rack ID.
        power_level *= rack_id;

        // Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
        power_level = (power_level / 100) % 10;

        // Subtract 5 from the power level.
        power_level -= 5;

        Cell { power_level }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_new() {
        assert_eq!(Cell::new(8, &Coordinate { x: 3, y: 5 }), Cell { power_level: 4 });
        assert_eq!(Cell::new(57, &Coordinate { x: 122, y: 79 }), Cell { power_level: -5 });
        assert_eq!(Cell::new(39, &Coordinate { x: 217, y: 196 }), Cell { power_level: 0 });
        assert_eq!(Cell::new(71, &Coordinate { x: 101, y: 153 }), Cell { power_level: 4 });
    }
}
