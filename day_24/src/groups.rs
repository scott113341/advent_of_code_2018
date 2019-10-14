use regex::Regex;
use std::cmp::Reverse;
use std::collections::{HashSet, HashMap};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Battle {
    pub immune_groups: Vec<Group>,
    pub infection_groups: Vec<Group>,
}

impl Battle {
    pub fn fight(&mut self) {
        let mut done = false;

        while !done {
            // TARGET SELECTION PHASE
            let mut target_selection_order: Vec<&Group> = self.immune_groups
                .iter()
                .chain(self.infection_groups.iter())
                .collect();
            target_selection_order.sort_by_key(|g| Reverse((g.effective_power(), g.initiative)));

            let mut targets = HashMap::new();
            let mut chosen = HashSet::new();

            for group in target_selection_order {
                let mut target_groups = match group.army_type {
                    ArmyType::Immune => self.infection_groups.clone(),
                    ArmyType::Infection => self.immune_groups.clone(),
                };

                target_groups.sort_by_key(|tg| Reverse((
                    Battle::damage(group, tg),
                    tg.effective_power(),
                    tg.initiative,
                )));

                for target_group in target_groups {
                    if Battle::damage(group, &target_group) == 0 {
                        continue;
                    }

                    if chosen.contains(&(target_group.army_type, target_group.id)) {
                        continue;
                    }

                    targets.insert((group.army_type, group.id), (target_group.army_type, target_group.id));
                    chosen.insert((target_group.army_type, target_group.id));
                    break;
                }
            }

            // BATTLE PHASE
            let mut battle_order: Vec<Group> = self.immune_groups
                .iter()
                .chain(self.infection_groups.iter())
                .map(|g| g.clone())
                .collect();
            battle_order.sort_by_key(|g| Reverse(g.initiative));

            let mut dead = HashSet::new();
            let mut new_units = HashMap::new();

            for group in &mut battle_order {
                // Skip if no target
                let target_group = targets.get(&(group.army_type, group.id));
                if target_group.is_none() { continue }
                let (target_group_type, target_group_id) = target_group.unwrap();

                // Get the actual group
                let groups = match target_group_type {
                    ArmyType::Immune => &mut self.immune_groups,
                    ArmyType::Infection => &mut self.infection_groups,
                };
                let target_group = groups.iter_mut().find(|g| g.id == *target_group_id);
                if target_group.is_none() { continue }
                let target_group = target_group.unwrap();

                // Skip if this group is dead
                if dead.contains(&(group.army_type, group.id)) { continue }

                // Mutate this group's unit count
                if let Some(updated_units) = new_units.get(&(group.army_type, group.id)) {
                    group.units = *updated_units;
                }

                let damage = Battle::damage(&group, &target_group);
                let group_hp = target_group.units * target_group.hit_points;

                if damage >= group_hp {
                    dead.insert((target_group.army_type, target_group.id));
                } else {
                    let units = (group_hp - damage + target_group.hit_points - 1) / target_group.hit_points;
                    new_units.insert((target_group.army_type, target_group.id), units);
                }
            }

            for (army_type, id) in &dead {
                let groups = match army_type {
                    ArmyType::Immune => &mut self.immune_groups,
                    ArmyType::Infection => &mut self.infection_groups,
                };

                if let Some(group) = groups.iter().find(|g| g.id == *id) {
                    let group = group.clone();
                    groups.remove_item(&group);
                }
            }

            for ((army_type, id), units) in &new_units {
                if let Some(group) = self.get_group_mut(*army_type, *id) {
                    group.units = *units;
                }
            }

            done = self.immune_groups.is_empty() || self.infection_groups.is_empty();
        }
    }

    fn get_group_mut(&mut self, army_type: ArmyType, id: usize) -> Option<&mut Group> {
        match army_type {
            ArmyType::Immune => self.immune_groups.iter_mut().find(|g| g.id == id),
            ArmyType::Infection => self.infection_groups.iter_mut().find(|g| g.id == id),
        }
    }

    fn damage(attacking_group: &Group, defending_group: &Group) -> usize {
        if defending_group.immunities.contains(&attacking_group.attack_type) {
            0
        } else if defending_group.weaknesses.contains(&attacking_group.attack_type) {
            attacking_group.effective_power() * 2
        } else {
            attacking_group.effective_power()
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Group {
    pub id: usize,
    pub army_type: ArmyType,
    pub units: usize,
    pub hit_points: usize,
    pub attack_damage: usize,
    pub attack_type: AttackType,
    pub initiative: usize,
    pub weaknesses: HashSet<AttackType>,
    pub immunities: HashSet<AttackType>,
}

static IMMUNE_GROUP_ID: AtomicUsize = AtomicUsize::new(1);
static INFECTION_GROUP_ID: AtomicUsize = AtomicUsize::new(1);

impl Group {
    pub fn parse(army_type: ArmyType, line: &str) -> Group {
        let re = Regex::new(r"(?P<units>\d+) units each with (?P<hit_points>\d+) hit points.*?with an attack that does (?P<attack_damage>\d+) (?P<attack_type>\w+) damage at initiative (?P<initiative>\d+)").unwrap();
        let caps = re.captures(line).unwrap();

        let weaknesses = Group::parse_attacks(
            Regex::new(r"\(.*weak to ([\w,\s]+?)[;)]").unwrap(),
            line,
        );

        let immunities = Group::parse_attacks(
            Regex::new(r"\(.*immune to ([\w,\s]+?)[;)]").unwrap(),
            line,
        );

        let id = match army_type {
            ArmyType::Immune => IMMUNE_GROUP_ID.fetch_add(1, Ordering::SeqCst),
            ArmyType::Infection => INFECTION_GROUP_ID.fetch_add(1, Ordering::SeqCst),
        };

        Group {
            id,
            army_type,
            units: caps["units"].parse().unwrap(),
            hit_points: caps["hit_points"].parse().unwrap(),
            attack_damage: caps["attack_damage"].parse().unwrap(),
            attack_type: AttackType::from_string(&caps["attack_type"]),
            initiative: caps["initiative"].parse().unwrap(),
            weaknesses,
            immunities,
        }
    }

    fn parse_attacks(regex: Regex, line: &str) -> HashSet<AttackType> {
        match regex.captures(line) {
            Some(cap) => {
                cap[1]
                    .to_string()
                    .split(", ")
                    .map(|s| AttackType::from_string(s))
                    .collect()
            },
            None => [].iter().cloned().collect(),
        }
    }

    pub fn effective_power(&self) -> usize {
        self.units * self.attack_damage
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ArmyType {
    Immune,
    Infection,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum AttackType {
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}

impl AttackType {
    pub fn from_string(attack_type: &str) -> AttackType {
        use AttackType::*;

        match attack_type {
            "bludgeoning" => Bludgeoning,
            "cold" => Cold,
            "fire" => Fire,
            "radiation" => Radiation,
            "slashing" => Slashing,
            _ => panic!("Dunno what is {}", attack_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_parse() {
        use ArmyType::*;
        use AttackType::*;

        let group = Group::parse(Immune, &"18 units each with 729 hit points (weak to fire; immune to cold, slashing) with an attack that does 8 radiation damage at initiative 10");

        assert_eq!(group.army_type, Immune);
        assert_eq!(group.units, 18);
        assert_eq!(group.hit_points, 729);
        assert_eq!(group.attack_damage, 8);
        assert_eq!(group.attack_type, Radiation);
        assert_eq!(group.initiative, 10);
        assert_eq!(group.weaknesses, [Fire].iter().cloned().collect());
        assert_eq!(group.immunities, [Cold, Slashing].iter().cloned().collect());
    }

    #[test]
    fn test_battle_fight() {
        let (immune_groups, infection_groups) = crate::tests::example_groups();

        let mut battle = Battle {
            immune_groups,
            infection_groups,
        };

        battle.fight();

        assert_eq!(battle.immune_groups.len(), 0);
        assert_eq!(battle.infection_groups.len(), 2);

        let mut unit_counts: Vec<usize> = battle.infection_groups.iter().map(|g| g.units).collect();
        unit_counts.sort();
        assert_eq!(unit_counts, vec![782, 4434]);
    }
}
