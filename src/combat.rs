use crate::configs;
use crate::character;
use character::Character;
use crate::modifiers::Aura; // used by tests
use crate::modifiers::Ability; // used by tests
use crate::targeting;
use rand;

// Loops through teams and pushes a vector sorted by character's speed
pub fn calculate_initiative(hero_team: &Vec<Character>, villain_team: &Vec<Character>) -> Vec<(i32, String, usize)> {
    let mut initiative = Vec::with_capacity(2usize * configs::TEAM_SIZE); // 2 teams
    for (index, character) in hero_team.iter().enumerate() {
        if ! character.is_dead() {
            initiative.push((character.speed, String::from("hero"), index));
        }
    }
    for (index, character) in villain_team.iter().enumerate() {
        if ! character.is_dead() {
            initiative.push((character.speed, String::from("villain"), index));
        }
    }
    // Sort by speed score; sort is ASCENDING so we can pop elements off
    initiative.sort_by_key(|i| i.0);
    return initiative
}

// Call out to Targeting module to decide targets, then attacks targets (per character)
fn attack(attacker: &mut Character, enemy_team: &mut Vec<Character>, enemy_team_alive: Vec<bool>) {
    let attack_type: &str = &attacker.attack_type;
    let targeting_data: Vec<bool> = targeting::attack_type_to_coordinates(attack_type, enemy_team_alive, &attacker.tags);
    for i in 0..enemy_team.len() {
        if targeting_data[i] { // If attacker should attack target
            // Check for crit on attack
            let roll_to_crit: f64 = rand::random::<f64>();
            let mut crit: bool = false;
            if roll_to_crit < attacker.critical_chance {
                crit = true;
            }
            // Make attack(s)
            attacker.print_attacking(&enemy_team[i].name);
            if crit {
                enemy_team[i].take_damage(attacker.power * configs::CRITICAL_MULTIPLIER, crit);
            } else {
                enemy_team[i].take_damage(attacker.power, crit);
            }
            if enemy_team[i].is_dead() {
                enemy_team[i].print_died();
            }
        }
    }
}

// Loop through in initiative order and attack, then check exit conditions
fn run_combat_round(mut initiative_order: Vec<(i32, String, usize)>, hero_team: &mut Vec<Character>, villain_team: &mut Vec<Character>) -> bool {
    for _i in 0..initiative_order.len() {
        let initiative_metadata = initiative_order.pop().unwrap();
        let hero_index: usize = initiative_metadata.2;
        let team_assignment = initiative_metadata.1;

        // Construct vectors of living characters to pass to the targeting function
        let mut hero_team_alive: Vec<bool> = Vec::with_capacity(configs::TEAM_SIZE);
        for hero in hero_team.iter() {
            if hero.is_dead() {
                hero_team_alive.push(false);
            } else {
                hero_team_alive.push(true);
            }
        }
        let mut villain_team_alive: Vec<bool> = Vec::with_capacity(configs::TEAM_SIZE);
        for villain in villain_team.iter() {
            if villain.is_dead() {
                villain_team_alive.push(false);
            } else {
                villain_team_alive.push(true);
            }
        }

        // Route the attacks properly: heroes attack villains, villains attack heros
        if team_assignment == String::from("hero") {
            if ! hero_team[hero_index].is_dead() {
                attack(&mut hero_team[hero_index], villain_team, villain_team_alive);
            }
        } else {
            if ! villain_team[hero_index].is_dead() {
                attack(&mut villain_team[hero_index], hero_team, hero_team_alive);
            }
        }
    }
    // Check to see if either team has been completely wiped out
    let mut heroes_alive: bool = false;
    'outer1: for character in hero_team.iter_mut() {
        if ! character.is_dead() {
            heroes_alive = true;
            break 'outer1;
        }
    }
    let mut villains_alive: bool = false;
    'outer2: for character in villain_team.iter_mut() {
        if ! character.is_dead() {
            villains_alive = true;
            break 'outer2;
        }
    }
    let continue_combat: bool = heroes_alive && villains_alive;
    return continue_combat;
}

pub fn run_combat(hero_team: &mut Vec<Character>, villain_team: &mut Vec<Character>) {
    let mut loop_count: u32 = 1;
    loop {
        println!("\n# --- ROUND {} --- #", loop_count);
        let init = calculate_initiative(&hero_team, &villain_team);
        let continue_combat: bool = run_combat_round(init, hero_team, villain_team);
        if ! continue_combat {
            break;
        }
        loop_count += 1;
    }
}

// Loop through the rounds until all members of one of the teams are dead
// pub fn run_combat(hero_team: &mut Vec<&mut Character>, villain_team: &mut Vec<&mut Character>) {
//     let mut loop_count: u32 = 1;
//     loop {
//         println!("\n# --- ROUND {} --- #", loop_count);
//         let init = calculate_initiative(&hero_team, &villain_team);
//         let continue_combat: bool = run_combat_round(init, hero_team, villain_team);
//         if ! continue_combat {
//             break;
//         }
//         loop_count += 1;
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    // Character Struct tests
    #[test]
    fn test_calculate_initiave() {
        const WILLIAM: &str = "William";
        const LOGAN: &str = "Logan";
        const CLASS: &str = "Warrior";
        const SPEED_WILLIAM: i32 = 1;
        const SPEED_LOGAN: i32 = 0;
        const HEALTH: i32 = 10;
        const POWER: i32 = 5;
        const CRITICAL_CHANCE: f64 = 0.0;
        let attack_type: String = "single".to_string();
        const DT: i32 = 1;
        const DR: f64 = 0.0;
        let tags_b: Vec<String> = vec!["null".to_string()];
        let tags_l: Vec<String> = vec!["null".to_string()];
        let auras_b: Vec<Aura> = vec![Aura::new("null", "null", 0.0)];
        let abilities_b: Vec<Ability> = vec![Ability::new("null", "null", 0.0)];
        let auras_l: Vec<Aura> = vec![Aura::new("null", "null", 0.0)];
        let abilities_l: Vec<Ability> = vec![Ability::new("null", "null", 0.0)];
        let expected_init: Vec<(i32, String, usize)> = vec![(0, "villain".to_string(), 0usize),
            (1, "hero".to_string(), 0usize)];

        // While mutability is unused in test, all of these must be mutable to satisfy fn conditions
        #[allow(unused_mut)]
        let mut bill = Character::new(WILLIAM, CLASS, SPEED_WILLIAM, HEALTH, POWER, CRITICAL_CHANCE, &attack_type, DT, DR, tags_b, auras_b, abilities_b);
        #[allow(unused_mut)]
        let mut logan = Character::new(LOGAN, CLASS, SPEED_LOGAN, HEALTH, POWER, CRITICAL_CHANCE, &attack_type, DT, DR, tags_l, auras_l, abilities_l);
        #[allow(unused_mut)]
        let mut heroes: Vec<Character> = vec![bill];
        #[allow(unused_mut)]
        let mut villains: Vec<Character> = vec![logan];
        let init = calculate_initiative(&heroes, &villains);

        assert_eq!(init, expected_init);
    }
}
