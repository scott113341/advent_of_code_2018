use crate::grid::{Pois, Grid};
use crate::part_1::compute_distances;

/// - Find each point with total distance to all coordinates less than 10,000
/// - Return count of these points
pub fn part_2(pois: &Pois, less_than: isize) -> usize {
    let max_x = pois.iter().map(|poi| poi.x).max().unwrap();
    let max_y = pois.iter().map(|poi| poi.y).max().unwrap();
    let mut grid = Grid::new(max_x + 1, max_y + 1);

    // Mutates each Point's poi_distances
    grid = compute_distances(pois, grid);

    let mut count = 0;

    for point in grid.grid {
        let sum: isize = point.poi_distances
            .iter()
            .map(|dis| dis.distance)
            .sum();

        if sum < less_than {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pois_and_grid() -> (Pois, Grid) {
        let pois = vec![
            Poi { name: "A".to_string(), x: 1, y: 1 },
            Poi { name: "B".to_string(), x: 1, y: 6 },
            Poi { name: "C".to_string(), x: 8, y: 3 },
            Poi { name: "D".to_string(), x: 3, y: 4 },
            Poi { name: "E".to_string(), x: 5, y: 5 },
            Poi { name: "F".to_string(), x: 8, y: 9 },
        ];
        let grid = Grid::new(9, 10);
        (pois, grid)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&pois_and_grid().0, 32), 16);
    }
}
