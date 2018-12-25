use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Marble {
    pub id: usize,
    pub prev: usize,
    pub next: usize,
}

pub struct GameState {
    pub player_count: usize,
    pub final_marble: usize,
    pub next_marble: usize,
    pub current_marble_idx: usize,
    pub removed_marbles: BTreeSet<usize>,
}

impl GameState {
    pub fn new(player_count: usize, final_marble: usize) -> GameState {
        GameState {
            player_count,
            final_marble,
            next_marble: 0,
            current_marble_idx: 0,
            removed_marbles: BTreeSet::new(),
        }
    }
}

impl IntoIterator for GameState {
    type Item = usize;
    type IntoIter = GameStateIterator;

    fn into_iter(self) -> Self::IntoIter {
        GameStateIterator { game_state: self }
    }
}

pub struct GameStateIterator {
    game_state: GameState,
}

impl Iterator for GameStateIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut gs = &mut self.game_state;

//        println!("\n\n####################");
//        dbg!(&gs.next_marble);
//        dbg!(&gs.current_marble_idx);
//        dbg!(&gs.removed_marbles);

        let next_marble = if let Some(next_removed) = gs.removed_marbles.iter().nth(0) {
            if *next_removed < gs.next_marble {
                let next = next_removed.to_owned();
                gs.removed_marbles.remove(&next);
                next
            } else {
                let next = gs.next_marble;
                gs.next_marble += 1;
                next
            }
        } else {
            let next = gs.next_marble;
            gs.next_marble += 1;
            next
        };

        if next_marble <= gs.final_marble {
            Some(next_marble)
        } else {
            None
        }
    }
}
