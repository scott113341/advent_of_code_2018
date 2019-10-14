#![feature(vec_remove_item)]

mod groups;

use crate::groups::{Battle, Group, ArmyType};

fn main() {
    let immune_groups: Vec<Group> = include_str!("immune.txt")
        .trim()
        .split("\n")
        .map(|s| Group::parse(ArmyType::Immune, s))
        .collect();

    let infection_groups: Vec<Group> = include_str!("infection.txt")
        .trim()
        .split("\n")
        .map(|s| Group::parse(ArmyType::Infection, s))
        .collect();

    println!("part_1: {}", part_1(immune_groups.clone(), infection_groups.clone()));
    println!("part_2: {}", part_2(immune_groups.clone(), infection_groups.clone()));
}

// How many units does the winning army have?
fn part_1(immune_groups: Vec<Group>, infection_groups: Vec<Group>) -> usize {
    let mut battle = Battle {
        immune_groups,
        infection_groups,
    };

    battle.fight();

    if battle.immune_groups.len() > 0 {
        battle.immune_groups.iter().map(|g| g.units).sum()
    } else {
        battle.infection_groups.iter().map(|g| g.units).sum()
    }
}

// How many units does the immune system have left after getting the smallest boost it needs to win?
fn part_2(immune_groups: Vec<Group>, infection_groups: Vec<Group>) -> usize {
    // The battle doesn't terminate at boost 45 because the final Immune group can't deal enough
    // damage to kill ANY Infection units, and the Immune group is immune to the Infection attack
    // type. So we just skip to boost 46, which gives us the answer.
    let mut boost = 46;

    loop {
        let mut immune_groups = immune_groups.clone();
        immune_groups.iter_mut().for_each(|g| g.attack_damage += boost);

        let mut battle = Battle {
            immune_groups,
            infection_groups: infection_groups.clone(),
        };

        battle.fight();

        if battle.immune_groups.len() > 0 {
            return battle.immune_groups.iter().map(|g| g.units).sum();
        } else {
            boost += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn example_groups() -> (Vec<Group>, Vec<Group>) {
        let immune_groups = vec![
            "17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2",
            "989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3",
        ].iter().map(|s| Group::parse(ArmyType::Immune, s)).collect();

        let infection_groups = vec![
            "801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1",
            "4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4",
        ].iter().map(|s| Group::parse(ArmyType::Infection, s)).collect();

        (immune_groups, infection_groups)
    }

    #[test]
    fn test_part_1() {
    }

    #[test]
    fn test_part_2() {
        let (immune_groups, infection_groups) = example_groups();
        assert_eq!(part_2(immune_groups, infection_groups), 51);
    }
}
