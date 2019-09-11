use std::collections::{BTreeMap, HashSet};

pub struct Maze {
    grid: Grid,
}

type Grid = BTreeMap<Coordinate, HashSet<Coordinate>>;

type Coordinate = (isize, isize);

impl Maze {
    pub fn new(pattern: &String) -> Maze {
        let mut stack: Vec<(isize, isize)> = vec![];
        let mut grid: Grid = BTreeMap::new();

        let mut current = (0, 0);

        for c in pattern.chars() {
            let previous = current.clone();

            match c {
                '^' => continue,
                '$' => continue,
                'N' => current = (current.0, current.1 + 1),
                'S' => current = (current.0, current.1 - 1),
                'E' => current = (current.0 + 1, current.1),
                'W' => current = (current.0 - 1, current.1),
                '(' => {
                    stack.push(current);
                    continue;
                },
                '|' => {
                    current = *stack.last().unwrap();
                    continue;
                },
                ')' => {
                    current = stack.pop().unwrap();
                    continue;
                },
                _ => panic!(),
            }

            let forward = grid.entry(previous).or_insert_with(|| HashSet::with_capacity(4));
            forward.insert(current);

            let backward = grid.entry(current).or_insert_with(|| HashSet::with_capacity(4));
            backward.insert(previous);
        }

        Maze { grid }
    }

    pub fn most_doors(&self) -> usize {
        let mut visited = HashSet::new();

        let mut doors = 0;
        let mut visit_next = vec![(0, 0)];

        while visit_next.len() > 0 {
            let mut new_visit_next = vec![];

            for current in &visit_next {
                visited.insert(*current);

                for next_node in self.grid.get(&current).unwrap() {
                    if !visited.contains(next_node) {
                        new_visit_next.push(next_node.clone());
                    }
                }
            }

            if new_visit_next.len() > 0 {
                doors += 1;
            }

            visit_next = new_visit_next;
        }

        doors
    }

    pub fn at_least_n_doors_away(&self, n_doors: usize) -> usize {
        let mut visited = HashSet::new();
        let mut visited_after_n_doors = HashSet::new();

        let mut doors = 0;
        let mut visit_next = vec![(0, 0)];

        while visit_next.len() > 0 {
            let mut new_visit_next = vec![];

            for current in &visit_next {
                visited.insert(*current);

                for next_node in self.grid.get(&current).unwrap() {
                    if !visited.contains(next_node) {
                        new_visit_next.push(next_node.clone());

                        if doors + 1 >= n_doors {
                            visited_after_n_doors.insert(*next_node);
                        }
                    }
                }
            }

            if new_visit_next.len() > 0 {
                doors += 1;
            }

            visit_next = new_visit_next;
        }

        visited_after_n_doors.len()
    }
}
