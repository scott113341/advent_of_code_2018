use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;

pub struct System {
    pub nodes: Nodes,
    pub carts: Carts,
    pub tick: usize,
}

pub type Nodes = HashMap<NodeCoordinate, Node>;
pub type Carts = BTreeMap<NodeCoordinate, Cart>;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
pub struct NodeCoordinate {
    pub y: usize,
    pub x: usize,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Node {
    StraightLr {
        left: NodeCoordinate,
        right: NodeCoordinate,
    },
    StraightUd {
        up: NodeCoordinate,
        down: NodeCoordinate,
    },
    CurveUr {
        up: NodeCoordinate,
        right: NodeCoordinate,
    },
    CurveUl {
        up: NodeCoordinate,
        left: NodeCoordinate,
    },
    CurveDl {
        down: NodeCoordinate,
        left: NodeCoordinate,
    },
    CurveDr {
        down: NodeCoordinate,
        right: NodeCoordinate,
    },
    Intersection {
        up: NodeCoordinate,
        down: NodeCoordinate,
        left: NodeCoordinate,
        right: NodeCoordinate,
    },
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Cart {
    pub direction: Direction,
    pub next_turn: TurnDirection,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TurnDirection {
    Left,
    Right,
    Straight,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl System {
    pub fn parse(lines: &Vec<&str>) -> System {
        let mut nodes: Nodes = HashMap::new();
        let mut carts: Carts = BTreeMap::new();

        for (y, line) in lines.iter().enumerate() {
            let mut chars_iter = line.chars().enumerate().peekable();

            // Process each character in the line
            loop {
                let next_char = chars_iter.next();
                if next_char == None { break }
                let (x, c) = next_char.unwrap();

                let coord = NodeCoordinate { y, x };
                let up = || NodeCoordinate { y: y - 1, x };
                let right = || NodeCoordinate { y, x: x + 1 };
                let down = || NodeCoordinate { y: y + 1, x };
                let left = || NodeCoordinate { y, x: x - 1 };

                match c {
                    '-' => {
                        nodes.insert(
                            coord,
                            Node::StraightLr { right: right(), left: left() },
                        );
                    },
                    '/' => {
                        match chars_iter.peek() {
                            Some((_, '-')) | Some((_, '+')) | Some((_, '>')) | Some((_, '<')) => {
                                nodes.insert(
                                    coord,
                                    Node::CurveDr { right: right(), down: down() },
                                );
                            },
                            _ => {
                                nodes.insert(
                                    coord,
                                    Node::CurveUl { up: up(), left: left() },
                                );
                            },
                        }
                    },
                    '\\' => {
                        match chars_iter.peek() {
                            Some((_, '-')) | Some((_, '+')) | Some((_, '>')) | Some((_, '<')) => {
                                nodes.insert(
                                    coord,
                                    Node::CurveUr { up: up(), right: right() },
                                );
                            },
                            _ => {
                                nodes.insert(
                                    coord,
                                    Node::CurveDl { down: down(), left: left() },
                                );
                            },
                        }
                    },
                    '|' => {
                        nodes.insert(
                            coord,
                            Node::StraightUd { up: up(), down: down() },
                        );
                    },
                    '+' => {
                        nodes.insert(
                            coord,
                            Node::Intersection {
                                up: up(),
                                right: right(),
                                down: down(),
                                left: left(),
                            },
                        );
                    },
                    '^' => {
                        nodes.insert(
                            coord,
                            Node::StraightUd { up: up(), down: down() },
                        );
                        carts.insert(
                            coord,
                            Cart { direction: Direction::Up, next_turn: TurnDirection::Left },
                        );
                    },
                    'v' => {
                        nodes.insert(
                            coord,
                            Node::StraightUd { up: up(), down: down() },
                        );
                        carts.insert(
                            coord,
                            Cart { direction: Direction::Down, next_turn: TurnDirection::Left },
                        );
                    },
                    '>' => {
                        nodes.insert(
                            coord,
                            Node::StraightLr { right: right(), left: left() },
                        );
                        carts.insert(
                            coord,
                            Cart { direction: Direction::Right, next_turn: TurnDirection::Left },
                        );
                    },
                    '<' => {
                        nodes.insert(
                            coord,
                            Node::StraightLr { right: right(), left: left() },
                        );
                        carts.insert(
                            coord,
                            Cart { direction: Direction::Left, next_turn: TurnDirection::Left },
                        );
                    },
                    ' ' => {},
                    _ => panic!("Unmatched character!"),
                };
            }
        }

        System {
            nodes,
            carts,
            tick: 0,
        }
    }

    pub fn next_coord(coord: &NodeCoordinate, direction: &Direction) -> NodeCoordinate {
        let mut y = coord.y;
        let mut x = coord.x;

        if *direction == Direction::Up { y -= 1 };
        if *direction == Direction::Down { y += 1 };
        if *direction == Direction::Right { x += 1 };
        if *direction == Direction::Left { x -= 1 };

        NodeCoordinate { y, x }
    }

    pub fn next_tick(&mut self) -> HashSet<NodeCoordinate> {
        let mut new_carts = self.carts.clone();
        let mut crashed_coords = HashSet::new();

        for (coord, cart) in &self.carts {
            let next_coord = Self::next_coord(coord, &cart.direction);
            let next_node = self.nodes.get(&next_coord).unwrap();

            let (new_direction, new_next_turn) = match next_node {
                Node::Intersection { .. } => {
                    // Find the cart's new direction
                    let new_direction = match cart.next_turn {
                        TurnDirection::Left => match cart.direction {
                            Direction::Up => Direction::Left,
                            Direction::Right => Direction::Up,
                            Direction::Down => Direction::Right,
                            Direction::Left => Direction::Down,
                        },
                        TurnDirection::Right => match cart.direction {
                            Direction::Up => Direction::Right,
                            Direction::Right => Direction::Down,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                        },
                        TurnDirection::Straight => cart.direction.clone(),
                    };

                    // Find the cart's new next_turn
                    let old_next_turn = &cart.next_turn;
                    let new_next_turn = match old_next_turn {
                        TurnDirection::Left => TurnDirection::Straight,
                        TurnDirection::Straight => TurnDirection::Right,
                        TurnDirection::Right => TurnDirection::Left,
                    };

                    (new_direction, new_next_turn)
                },
                Node::StraightLr { .. } | Node::StraightUd { .. } => {
                    (cart.direction.clone(), cart.next_turn.clone())
                },
                Node::CurveUr { .. } => {
                    let new_direction = match cart.direction {
                        Direction::Left => Direction::Up,
                        _ => Direction::Right,
                    };
                    (new_direction, cart.next_turn.clone())
                },
                Node::CurveUl { .. } => {
                    let new_direction = match cart.direction {
                        Direction::Right => Direction::Up,
                        _ => Direction::Left,
                    };
                    (new_direction, cart.next_turn.clone())
                },
                Node::CurveDl { .. } => {
                    let new_direction = match cart.direction {
                        Direction::Right => Direction::Down,
                        _ => Direction::Left,
                    };
                    (new_direction, cart.next_turn.clone())
                },
                Node::CurveDr { .. } => {
                    let new_direction = match cart.direction {
                        Direction::Left => Direction::Down,
                        _ => Direction::Right,
                    };
                    (new_direction, cart.next_turn.clone())
                },
            };

            if new_carts.get(&next_coord).is_some() {
                // A cart already exists in this cart's next position
                //   - Remove both carts
                //   - Save the crash coordinate to "crash_coords"
                new_carts.remove(&coord).unwrap();
                new_carts.remove(&next_coord).unwrap();
                crashed_coords.insert(next_coord);
            } else {
                // The cart at new_carts[coord] may already have been removed during a collision,
                // so only insert if there was an existing cart to remove
                new_carts.remove(&coord).and_then(|_| {
                    new_carts.insert(
                        next_coord,
                        Cart {
                            direction: new_direction,
                            next_turn: new_next_turn,
                        },
                    )
                });
            }
        }

        self.carts = new_carts;
        self.tick += 1;
        crashed_coords
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_parse() {
        let system = System::parse(&crate::tests::test_input_1());

        assert_eq!(system.nodes.len(), 48);
        assert_eq!(
            system.nodes[&NodeCoordinate { y: 0, x: 0 }],
            Node::CurveDr {
                down: NodeCoordinate { y: 1, x: 0 },
                right: NodeCoordinate { y: 0, x: 1 },
            }
        );
        assert_eq!(
            system.nodes[&NodeCoordinate { y: 0, x: 1 }],
            Node::StraightLr {
                left: NodeCoordinate { y: 0, x: 0 },
                right: NodeCoordinate { y: 0, x: 2 },
            }
        );
        assert_eq!(
            system.nodes[&NodeCoordinate { y: 0, x: 4 }],
            Node::CurveDl {
                down: NodeCoordinate { y: 1, x: 4 },
                left: NodeCoordinate { y: 0, x: 3 },
            }
        );
        assert_eq!(
            system.nodes[&NodeCoordinate { y: 2, x: 4 }],
            Node::Intersection {
                up: NodeCoordinate { y: 1, x: 4 },
                right: NodeCoordinate { y: 2, x: 5 },
                down: NodeCoordinate { y: 3, x: 4 },
                left: NodeCoordinate { y: 2, x: 3 },
            }
        );

        assert_eq!(system.carts.len(), 2);
        assert_eq!(
            system.carts[&NodeCoordinate { y: 0, x: 2 }],
            Cart {
                direction: Direction::Right,
                next_turn: TurnDirection::Left,
            },
        );
        assert_eq!(
            system.carts[&NodeCoordinate { y: 3, x: 9 }],
            Cart {
                direction: Direction::Down,
                next_turn: TurnDirection::Left,
            },
        );

        assert_eq!(system.tick, 0);
    }

    #[test]
    fn test_system_next_tick() {
        let mut system = System::parse(&crate::tests::test_input_1());

        assert_eq!(system.next_tick(), HashSet::new());
        assert_eq!(system.tick, 1);
        assert_eq!(
            system.carts[&NodeCoordinate { y: 0, x: 3 }],
            Cart {
                direction: Direction::Right,
                next_turn: TurnDirection::Left,
            },
        );
        assert_eq!(
            system.carts[&NodeCoordinate { y: 4, x: 9 }],
            Cart {
                direction: Direction::Right,
                next_turn: TurnDirection::Straight,
            },
        );

        assert_eq!(system.next_tick(), HashSet::new());
        assert_eq!(system.tick, 2);
        assert_eq!(
            system.carts[&NodeCoordinate { y: 0, x: 4 }],
            Cart {
                direction: Direction::Down,
                next_turn: TurnDirection::Left,
            },
        );
        assert_eq!(
            system.carts[&NodeCoordinate { y: 4, x: 10 }],
            Cart {
                direction: Direction::Right,
                next_turn: TurnDirection::Straight,
            },
        );

        while system.tick < 13 {
            assert_eq!(system.next_tick(), HashSet::new());
        }

        assert_eq!(system.tick, 13);
        assert_eq!(
            system.carts[&NodeCoordinate { y: 2, x: 7 }],
            Cart {
                direction: Direction::Down,
                next_turn: TurnDirection::Right,
            },
        );
        assert_eq!(
            system.carts[&NodeCoordinate { y: 4, x: 7 }],
            Cart {
                direction: Direction::Up,
                next_turn: TurnDirection::Left,
            },
        );

        assert!(system.next_tick().contains(&NodeCoordinate { y: 3, x: 7 }));
        assert_eq!(system.tick, 14);
    }

    #[test]
    fn test_node_coordinate_ordering() {
        let mut coords = vec![
            NodeCoordinate { y: 1, x: 1 },
            NodeCoordinate { y: 0, x: 5 },
            NodeCoordinate { y: 1, x: 4 },
            NodeCoordinate { y: 0, x: 0 },
        ];
        coords.sort();
        assert_eq!(coords[0], NodeCoordinate { y: 0, x: 0 });
        assert_eq!(coords[1], NodeCoordinate { y: 0, x: 5 });
        assert_eq!(coords[2], NodeCoordinate { y: 1, x: 1 });
        assert_eq!(coords[3], NodeCoordinate { y: 1, x: 4 });
    }
}
