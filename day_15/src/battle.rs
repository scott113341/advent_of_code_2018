use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;

pub struct Battle {
    pub map: Map,
    pub round: usize,
    pub is_finished: bool,
}

impl Battle {
    /// Number of full rounds multiplied by sum of the hit points of all remaining units
    pub fn outcome(&self) -> usize {
        let hp = self.map.players
            .iter()
            .map(|(_point, player)| player.hp)
            .sum::<isize>() as usize;
        self.round * hp
    }

    pub fn play_round(&mut self) {
        println!("\n\n\n\n########## STARTING ROUND {} ##########", self.round + 1);

        for (point, player) in self.map.players.clone().iter() {
            let mut point = *point;
            let adjacent_points = self.map.adjacent_points(&point);

            println!("\n=> Playing {:?} {} at {}", player.player_type, player.id, point);

            // Skip turn if dead
            if self.map.players.iter().find(|(_, pl)| pl.id == player.id).is_none() {
                println!("   Dead!");
                continue;
            }

            // CHECK IF COMBAT FINISHED
            let player_types = self.map.players
                .iter()
                .map(|(_point, player)| player.player_type.clone())
                .collect::<HashSet<_>>();
            if player_types.len() == 1 {
                println!("   COMBAT FINISHED!");
                self.is_finished = true;
                break;
            }

            // MOVE
            let new_point = (|| {
                // Skip movement if adjacent to enemy
                let adjacent_enemy = adjacent_points.iter()
                    .filter(|point| self.map.is_enemy_at(point, &player.player_type))
                    .nth(0);
                if adjacent_enemy.is_some() {
                    println!("   Has adjacent enemies, not moving");
                    return None;
                }

                // Skip movement if no moves are available
                let open_adjacent_points = adjacent_points.iter()
                    .filter(|p| self.map.players.get(p).is_none())
                    .collect::<Vec<&Point>>();
                if open_adjacent_points.is_empty() {
                    println!("   Has no moves");
                    return None;
                }

                // Find next move
                if let Some(next_point) = self.map.next_move(&point, player) {
                    println!("   Moved to {}", next_point);
                    let player = self.map.players.remove(&point).unwrap();
                    self.map.players.insert(next_point, player);
                    Some(next_point)
                } else {
                    println!("   Searched, but nowhere to move");
                    None
                }
            })();

            // Reassign the "point" variable if we moved
            if let Some(new_point) = new_point {
                point = new_point;
            }

            // ATTACK
            let adjacent_points = self.map.adjacent_points(&point);
            let adjacent_enemy = adjacent_points.iter()
                .filter(|point| self.map.is_enemy_at(point, &player.player_type))
                .min_by_key(|point| self.map.players.get(point).unwrap().hp);
            if let Some(adjacent_enemy) = adjacent_enemy {
                println!("   Attacking adjacent enemy at {}", adjacent_enemy);
                let enemy = self.map.players.get(adjacent_enemy).unwrap();
                if enemy.hp <= player.ap {
                    self.map.players.remove(adjacent_enemy).unwrap();
                    println!("   Killed enemy!");
                } else {
                    let enemy = self.map.players.get_mut(adjacent_enemy).unwrap();
                    enemy.hp -= player.ap;
                    println!("   Attacked enemy to {} hp", enemy.hp);
                }
            } else {
                println!("   No adjacent enemy");
            }
        }

        println!("\n{:?}", &self.map);

        if self.is_finished {
            println!("FINISHED BATTLE PARTWAY THROUGH ROUND {}", self.round + 1);
        } else {
            self.round += 1;
            println!("FINISHED ROUND {}", self.round);
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct Point {
    pub row: isize,
    pub col: isize,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.row, self.col)
    }
}

pub fn pt(row: isize, col: isize) -> Point {
    Point { row, col }
}


/// PLAYER
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Player {
    pub id: usize,
    pub player_type: PlayerType,
    pub hp: isize,
    pub ap: isize,
}

#[derive(Eq, PartialEq, Clone, Hash)]
pub enum PlayerType {
    Elf,
    Goblin,
}

impl fmt::Debug for PlayerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlayerType::Elf => write!(f, "E"),
            PlayerType::Goblin => write!(f, "G"),
        }
    }
}


/// MAP
pub struct Map {
    pub players: Players,
    pub grid: Grid,
}
pub type IsWall = bool;
pub type Players = BTreeMap<Point, Player>;
pub type Grid = BTreeMap<Point, IsWall>;

impl Map {
    pub fn parse(lines: &Vec<&str>) -> Map {
        let mut players = BTreeMap::new();
        let mut grid = BTreeMap::new();
        let mut id = 0;

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let row = row as isize;
                let col = col as isize;

                match ch {
                    'E' => {
                        grid.insert(pt(row, col), false);
                        players.insert(pt(row, col), Player {
                            id,
                            player_type: PlayerType::Elf,
                            hp: 200,
                            ap: 3,
                        });
                        id += 1;
                    },
                    'G' => {
                        grid.insert(pt(row, col), false);
                        players.insert(pt(row, col), Player {
                            id,
                            player_type: PlayerType::Goblin,
                            hp: 200,
                            ap: 3,
                        });
                        id += 1;
                    },
                    '.' => {
                        grid.insert(pt(row, col), false);
                    },
                    '#' => {
                        grid.insert(pt(row, col), true);
                    },
                    _ => unreachable!(),
                }
            }
        }

        Map { players, grid }
    }

    /// Returns "reading order"-ed adjacent points that exist and are not walls
    pub fn adjacent_points(&self, from_point: &Point) -> Vec<Point> {
        let mut points = Vec::with_capacity(4);
        let up = pt(from_point.row - 1, from_point.col + 0);
        let left = pt(from_point.row + 0, from_point.col - 1);
        let right = pt(from_point.row + 0, from_point.col + 1);
        let down = pt(from_point.row + 1, from_point.col + 0);

        self.grid.get(&up).map(|is_wall| if !*is_wall { points.push(up) });
        self.grid.get(&left).map(|is_wall| if !*is_wall { points.push(left) });
        self.grid.get(&right).map(|is_wall| if !*is_wall { points.push(right) });
        self.grid.get(&down).map(|is_wall| if !*is_wall { points.push(down) });

        points
    }

    pub fn is_enemy_at(&self, point: &Point, self_type: &PlayerType) -> bool {
        self.players
            .get(point)
            .filter(|player| player.player_type != *self_type)
            .is_some()
    }

    pub fn next_move(&self, from_point: &Point, player: &Player) -> Option<Point> {
        type VisitPoint = (Point, Point);

        // HashMap<the point, previous point>
        let mut visited_points: HashMap<Point, Option<Point>> = HashMap::new();
        visited_points.insert(*from_point, None);

        // Specify the first set of points to visit; we'll spider out and add more later
        let mut points_to_visit: VecDeque<VisitPoint> = VecDeque::new();
        for p in self.adjacent_points(from_point) {
            points_to_visit.push_back((p, *from_point));
        }

        // The next set of points to visit as we discover them
        let mut next_points_to_visit: Vec<VisitPoint> = Vec::new();

        // Equidistant enemy accessibility points
        let mut enemies = Vec::new();

        let final_move = loop {
            if let Some((chk_point, prev_point)) = points_to_visit.pop_front() {

                // Skip checking points that have been visited already
                if visited_points.contains_key(&chk_point) {
                    continue;
                }

                // Visit all of this point's adjacent points the next iteration, as long as they
                // have not been visited and are not occupied
                for p in self.adjacent_points(&chk_point) {
                    if !visited_points.contains_key(&p) && !self.players.contains_key(&chk_point) {
                        next_points_to_visit.push((p, chk_point));
                    }
                }

                // Mark this point as visited
                visited_points.insert(chk_point, Some(prev_point));

                // Finally check the point.  If there's an enemy at it, add it to the "enemies"
                // vector for comparison against other equidistant enemies.
                let has_enemy = self.is_enemy_at(&chk_point, &player.player_type);
                if has_enemy {
                    enemies.push(prev_point);
                } else {
                }
            } else {
                if !enemies.is_empty() {
                    enemies.sort();
                    break Some(*enemies.first().unwrap());
                } else if next_points_to_visit.is_empty() {
                    break None;
                } else {
                    enemies.clear();
                    next_points_to_visit.sort();
                    points_to_visit = next_points_to_visit.into_iter().collect();
                    next_points_to_visit = Vec::new();
                }
            }
        };

        // Return if there is no path to an enemy
        if final_move == None {
            return None;
        }

        // Follow the visited points back to the original from_point
        let mut next_move = final_move.unwrap();
        loop {
            let previous_move = visited_points.get(&next_move).unwrap().unwrap();
            if previous_move == *from_point {
                break;
            } else {
                next_move = previous_move;
            }
        }

        Some(next_move)
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut current_row = 0;
        let mut current_row_players: Vec<&Player> = Vec::new();

        for (point, is_wall) in self.grid.iter() {
            if point.row != current_row {
                for player in current_row_players.iter() {
                    write!(f, " {:?}({}),", player.player_type, player.hp)?;
                }
                write!(f, "\n")?;

                current_row = point.row;
                current_row_players.clear();
            }

            if *is_wall {
                write!(f, "#")?;
            } else {
                if let Some(player) = self.players.get(&point) {
                    current_row_players.push(player);
                    write!(f, "{:?}", player.player_type)?;
                } else {
                    write!(f, ".")?;
                }
            }
        }

        write!(f, "\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battle_play_round() {
        let mut battle = Battle {
            map: Map::parse(&super::super::tests::example_1()),
            round: 0,
            is_finished: false,
        };

        /*
        After 1 round:
        #######
        #..G..#   G(200)
        #...EG#   E(197), G(197)
        #.#G#G#   G(200), G(197)
        #...#E#   E(197)
        #.....#
        #######
        */
        battle.play_round();
        assert_eq!(battle.is_finished, false);
        assert_eq!(battle.round, 1);
        let (pt0, pl0) = battle.map.players.iter().nth(0).unwrap();
        assert_eq!(pt0, &pt(1, 3));
        assert_eq!(pl0.hp, 200);
        let (pt1, pl1) = battle.map.players.iter().nth(1).unwrap();
        assert_eq!(pt1, &pt(2, 4));
        assert_eq!(pl1.hp, 197);
        let (pt2, pl2) = battle.map.players.iter().nth(2).unwrap();
        assert_eq!(pt2, &pt(2, 5));
        assert_eq!(pl2.hp, 197);
        let (pt3, pl3) = battle.map.players.iter().nth(3).unwrap();
        assert_eq!(pt3, &pt(3, 3));
        assert_eq!(pl3.hp, 200);
        let (pt4, pl4) = battle.map.players.iter().nth(4).unwrap();
        assert_eq!(pt4, &pt(3, 5));
        assert_eq!(pl4.hp, 197);
        let (pt5, pl5) = battle.map.players.iter().nth(5).unwrap();
        assert_eq!(pt5, &pt(4, 5));
        assert_eq!(pl5.hp, 197);

        /*
        After 2 rounds:
        #######
        #...G.#   G(200)
        #..GEG#   G(200), E(188), G(194)
        #.#.#G#   G(194)
        #...#E#   E(194)
        #.....#
        #######
        */
        battle.play_round();
        assert_eq!(battle.is_finished, false);
        assert_eq!(battle.round, 2);
        let (pt0, pl0) = battle.map.players.iter().nth(0).unwrap();
        assert_eq!(pt0, &pt(1, 4));
        assert_eq!(pl0.hp, 200);
        let (pt1, pl1) = battle.map.players.iter().nth(1).unwrap();
        assert_eq!(pt1, &pt(2, 3));
        assert_eq!(pl1.hp, 200);
        let (pt2, pl2) = battle.map.players.iter().nth(2).unwrap();
        assert_eq!(pt2, &pt(2, 4));
        assert_eq!(pl2.hp, 188);
        let (pt3, pl3) = battle.map.players.iter().nth(3).unwrap();
        assert_eq!(pt3, &pt(2, 5));
        assert_eq!(pl3.hp, 194);
        let (pt4, pl4) = battle.map.players.iter().nth(4).unwrap();
        assert_eq!(pt4, &pt(3, 5));
        assert_eq!(pl4.hp, 194);
        let (pt5, pl5) = battle.map.players.iter().nth(5).unwrap();
        assert_eq!(pt5, &pt(4, 5));
        assert_eq!(pl5.hp, 194);

        /*
        After 28 rounds:
        #######
        #G....#   G(200)
        #.G...#   G(131)
        #.#.#G#   G(116)
        #...#E#   E(113)
        #....G#   G(200)
        #######
        */
        while battle.round < 28 { battle.play_round() }
        assert_eq!(battle.is_finished, false);
        assert_eq!(battle.round, 28);
        let (pt0, pl0) = battle.map.players.iter().nth(0).unwrap();
        assert_eq!(pt0, &pt(1, 1));
        assert_eq!(pl0.hp, 200);
        let (pt1, pl1) = battle.map.players.iter().nth(1).unwrap();
        assert_eq!(pt1, &pt(2, 2));
        assert_eq!(pl1.hp, 131);
        let (pt2, pl2) = battle.map.players.iter().nth(2).unwrap();
        assert_eq!(pt2, &pt(3, 5));
        assert_eq!(pl2.hp, 116);
        let (pt3, pl3) = battle.map.players.iter().nth(3).unwrap();
        assert_eq!(pt3, &pt(4, 5));
        assert_eq!(pl3.hp, 113);
        let (pt4, pl4) = battle.map.players.iter().nth(4).unwrap();
        assert_eq!(pt4, &pt(5, 5));
        assert_eq!(pl4.hp, 200);

        /*
        After 47 rounds:
        #######
        #G....#   G(200)
        #.G...#   G(131)
        #.#.#G#   G(59)
        #...#.#
        #....G#   G(200)
        #######
        */
        while battle.round < 47 { battle.play_round() }
        assert_eq!(battle.is_finished, false);
        assert_eq!(battle.round, 47);
        let (pt0, pl0) = battle.map.players.iter().nth(0).unwrap();
        assert_eq!(pt0, &pt(1, 1));
        assert_eq!(pl0.hp, 200);
        let (pt1, pl1) = battle.map.players.iter().nth(1).unwrap();
        assert_eq!(pt1, &pt(2, 2));
        assert_eq!(pl1.hp, 131);
        let (pt2, pl2) = battle.map.players.iter().nth(2).unwrap();
        assert_eq!(pt2, &pt(3, 5));
        assert_eq!(pl2.hp, 59);
        let (pt3, pl3) = battle.map.players.iter().nth(3).unwrap();
        assert_eq!(pt3, &pt(5, 5));
        assert_eq!(pl3.hp, 200);

        // Round ends immediately
        battle.play_round();
        assert_eq!(battle.is_finished, true);
        assert_eq!(battle.round, 47);
        assert_eq!(battle.outcome(), 27730);
    }

    #[test]
    fn test_pt() {
        assert_eq!(pt(2, 3), Point { row: 2, col: 3 });
    }

    #[test]
    fn test_map_parse() {
        let map = Map::parse(&super::super::tests::example_1());

        assert_eq!(map.grid.len(), 7 * 7);
        assert_eq!(map.grid.get(&pt(0, 0)).unwrap(), &true);
        assert_eq!(map.grid.get(&pt(1, 0)).unwrap(), &true);
        assert_eq!(map.grid.get(&pt(1, 1)).unwrap(), &false);
        assert_eq!(map.grid.get(&pt(1, 2)).unwrap(), &false);

        assert_eq!(map.players.len(), 6);
        assert_eq!(map.players.get(&pt(1, 2)).unwrap(), &Player {
            id: 0,
            player_type: PlayerType::Goblin,
            hp: 200,
            ap: 3,
        });
        assert_eq!(map.players.get(&pt(2, 4)).unwrap(), &Player {
            id: 1,
            player_type: PlayerType::Elf,
            hp: 200,
            ap: 3,
        });

        println!("{:?}", map);
    }

    #[test]
    fn test_map_adjacent_points() {
        let map = Map::parse(&super::super::tests::example_1());
        assert_eq!(map.adjacent_points(&pt(0, 0)), vec![]);
        assert_eq!(map.adjacent_points(&pt(0, 1)), vec![pt(1, 1)]);
        assert_eq!(map.adjacent_points(&pt(1, 1)), vec![pt(1, 2), pt(2, 1)]);
        assert_eq!(map.adjacent_points(&pt(2, 3)), vec![pt(1, 3), pt(2, 2), pt(2, 4), pt(3, 3)]);
        assert_eq!(map.adjacent_points(&pt(3, 3)), vec![pt(2, 3), pt(4, 3)]);
    }
}
