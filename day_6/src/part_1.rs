use std::collections::HashMap;

use crate::grid::{Pois, Grid, Distance};

/// - Place POIs on grid
/// - Compute closest POI for each point
/// - Which POI has the largest area?
///   - Throw out infinite POIs (grid infinite)
///   - Throw out points that are tied
/// 
/// - Construct grid: Vec<Point>
/// - For each Point, compute distance to each POI
/// - Sort POIs by area
///   - Throw out ties
///   - Throw out if any non-tied point touches edge of grid (infinite)
/// - (Check infinite) Throw out if any point touches the edge of grid (infinite)
pub fn part_1(pois: &Pois) -> usize {
    let max_x = pois.iter().map(|poi| poi.x).max().unwrap();
    let max_y = pois.iter().map(|poi| poi.y).max().unwrap();
    let mut grid = Grid::new(max_x + 1, max_y + 1);

    // Mutates each Point's poi_distances
    grid = compute_distances(pois, grid);

    let valid_area_counts = compute_valid_area_counts(pois, &grid);
    *valid_area_counts.values().max().unwrap()
}

pub fn compute_distances(pois: &Pois, mut grid: Grid) -> Grid {
    for point in grid.grid.iter_mut() {
        for poi in pois {
            let poi_distance = Distance {
                from_poi: poi.clone(),
                distance: (point.x - poi.x).abs() + (point.y - poi.y).abs(),
            };
            point.poi_distances.push(poi_distance);
        }

        // Sort once all are pushed
        point.poi_distances.sort_by_key(|poi_dis| poi_dis.distance);
    }

    grid
}

fn compute_valid_area_counts(pois: &Pois, grid: &Grid) -> HashMap<String, usize> {
    // Initialize a HashMap<PoiName, Count>.  Invalid POIs will be removed, and Points that are
    // tied in distance will not be counted.
    let mut valid_area_counts = HashMap::new();
    for poi in pois {
        valid_area_counts.insert(poi.name.clone(), 0);
    }

    // Iterate through each point, mutating valid_area_counts as necessary
    for point in &grid.grid {
        let is_tied = &point.poi_distances[0].distance == &point.poi_distances[1].distance;
        let is_edge =
            point.x == 0 || point.x == (&grid.x_size - 1)
            || point.y == 0 || point.y == (&grid.y_size - 1)
        ;

        match (is_tied, is_edge) {
            // Don't count this Point for any POI because it's tied
            (true,  _)  => (),

            // Throw out this POI because it's infinite
            (false, true)  => {
                valid_area_counts.remove(&point.poi_distances[0].from_poi.name);
                ()
            },

            // Count
            (false, false) => {
                let name = &point.poi_distances[0].from_poi.name;
                if let Some(count) = valid_area_counts.get_mut(name) {
                    *count += 1;
                }
                ()
            },
        }
    }

    valid_area_counts
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
    fn test_part_1() {
        assert_eq!(part_1(&pois_and_grid().0), 17);
    }

    #[test]
    fn test_compute_distances() {
        let (pois, mut grid) = pois_and_grid();
        grid = compute_distances(&pois, grid);

        let idx = 37; // Point at (1, 4)
        assert_eq!(grid.grid[idx].poi_distances.len(), 6);

        assert_eq!(grid.grid[idx].poi_distances[0].from_poi.name, "B".to_string());
        assert_eq!(grid.grid[idx].poi_distances[0].distance, 2);
        assert_eq!(grid.grid[idx].poi_distances[1].from_poi.name, "D".to_string());
        assert_eq!(grid.grid[idx].poi_distances[1].distance, 2);
        assert_eq!(grid.grid[idx].poi_distances[2].from_poi.name, "A".to_string());
        assert_eq!(grid.grid[idx].poi_distances[2].distance, 3);
        assert_eq!(grid.grid[idx].poi_distances[3].from_poi.name, "E".to_string());
        assert_eq!(grid.grid[idx].poi_distances[3].distance, 5);
        assert_eq!(grid.grid[idx].poi_distances[4].from_poi.name, "C".to_string());
        assert_eq!(grid.grid[idx].poi_distances[4].distance, 8);
        assert_eq!(grid.grid[idx].poi_distances[5].from_poi.name, "F".to_string());
        assert_eq!(grid.grid[idx].poi_distances[5].distance, 12);
    }
}
