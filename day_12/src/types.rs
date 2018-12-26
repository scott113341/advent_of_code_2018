use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt;

pub type PotId = isize;

pub struct Pot {
    pub has_plant: bool,
}

pub type Pots = BTreeMap<PotId, Pot>;

// LLCRR
#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Rule(pub bool, pub bool, pub bool, pub bool, pub bool);
pub type YieldsPlant = bool;
pub type Rules = HashMap<Rule, YieldsPlant>;

pub struct PotSim {
    pub generation: usize,
    pub pots: Pots,
    pub rules: Rules,
}

impl PotSim {
    pub fn next_generation(&mut self) {
        self.add_or_remove_empty_start_end_pots(5);

        let min_pot_id = self.pots.iter().nth(0).unwrap().0.to_owned();
        let max_pot_id = self.pots.iter().last().unwrap().0.to_owned();
        let mut next_has_plants = HashMap::new();

        // Populate the "next_has_plant" field for each Pot
        for (pot_id, _pot) in &self.pots {
            if *pot_id < min_pot_id + 2 { continue; }
            if *pot_id > max_pot_id - 2 { continue; }

            let rule = Rule(
                self.pots.get(&(pot_id - 2)).unwrap().has_plant,
                self.pots.get(&(pot_id - 1)).unwrap().has_plant,
                self.pots.get(&(pot_id + 0)).unwrap().has_plant,
                self.pots.get(&(pot_id + 1)).unwrap().has_plant,
                self.pots.get(&(pot_id + 2)).unwrap().has_plant,
            );

            // Look up the Rule and save whether this pot will have a plant next generation
            let yields_plant = match self.rules.get(&rule) {
                Some(yp) => *yp,
                None => false,
            };
            next_has_plants.insert(pot_id.to_owned(), yields_plant);
        }

        // Persist the "next" generation as the new current generation and reset the "next"
        for (pot_id, next_has_plant) in next_has_plants {
            self.pots.get_mut(&pot_id).unwrap().has_plant = next_has_plant;
        }

        self.generation += 1;
    }

    // Add or remove empty start and end pots to keep a 5-pot buffer.  Added after examining
    // later generations for Part 2 - it's a stable "glider" configuration, that goes in the
    // positive direction, so trimming pots from the start is useful.
    pub fn add_or_remove_empty_start_end_pots(&mut self, buffer: usize) {
        let ibuffer = buffer as isize;

        let empty_at_start = self.pots.iter().take_while(|(_, pot)| !pot.has_plant).count();
        let empty_at_end = self.pots.iter().rev().take_while(|(_, pot)| !pot.has_plant).count();
        let min_plant_id = *self.pots.iter().find(|(_, pot)| pot.has_plant).unwrap().0;
        let max_plant_id = *self.pots.iter().rev().find(|(_, pot)| pot.has_plant).unwrap().0;

        // Add or remove at start
        if empty_at_start < buffer {
            for pot_id in (min_plant_id - ibuffer)..min_plant_id {
                self.pots.insert(pot_id, Pot { has_plant: false });
            }
        } else if empty_at_start > buffer {
            let min_id = *self.pots.iter().nth(0).unwrap().0;
            for pot_id in min_id..(min_plant_id - ibuffer) {
                self.pots.remove(&pot_id);
            }
        }

        // Add or remove at end
        if empty_at_end < buffer {
            for pot_id in (max_plant_id + 1)..=(max_plant_id + ibuffer) {
                self.pots.insert(pot_id, Pot { has_plant: false });
            }
        } else if empty_at_end > buffer {
            for pot_id in (max_plant_id - (empty_at_end as isize) + ibuffer + 1)..=max_plant_id {
                self.pots.remove(&pot_id);
            }
        }
    }
}

impl fmt::Debug for PotSim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let plants = self.pots
            .iter()
            .map(|(pot_id, pot)| {
                let plant = if pot.has_plant { '#' } else { '.' };
                format!("{: >5} {}", pot_id, plant)
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", plants)
    }
}
