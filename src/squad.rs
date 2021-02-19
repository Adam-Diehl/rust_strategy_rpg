/*
A squad is a vector of characters (order matters!). Most combat operations involve looping over a
squad. This file holds the SquadConstructor struct, which consumes a vector of strings and returns
a vector of Character (structs), applying relevant auras and whatnot.
*/

use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::character;
use character::Character;
use crate::configs;
use crate::input;
use crate::modifiers::Apply;
use crate::modifiers::Aura;

#[derive(Deserialize)]
pub struct SquadConstructor {
    pub members: Vec<String>
}

// Read squad members from file
fn read_params_from_file<P: AsRef<Path>>(path: P) -> Result<SquadConstructor, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let params = serde_yaml::from_reader(reader)?;
    Ok(params)
}

impl SquadConstructor {
    fn new(
        members: Vec<String>
    ) -> SquadConstructor {
        SquadConstructor {
            members: members
        }
    }

    pub fn new_from_file(filepath: &str) -> Self {
        let squadc: SquadConstructor = read_params_from_file(filepath).unwrap();
        return squadc;
    }

    fn build_from_input() -> Self {
        let front_left = input::grab_input(&"Choose character for the front-left position: ");
        let front_right = input::grab_input(&"Choose character for the front-right position: ");
        let back_left = input::grab_input(&"Choose character for the back-left position: ");
        let back_right = input::grab_input(&"Choose character for the back-right position: ");
        let squad_constructor: Vec<String> = vec![front_left, front_right, back_left, back_right];
        return SquadConstructor::new(squad_constructor);
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{:?}", self.members)
    }
}

/* --------------------------------------------------------------------------------------------- */

fn apply_auras(mut squad: Vec<Character>) -> Vec<Character> {
    // Loops twice: first over the squad to collect all auras and specific targets,
    // and then again over the collected auras to apply them to the squad

    // Note that auras that apply to self are applied in the first pass
    let mut deferred_auras: Vec<Aura> = Vec::new();
    for character in squad.iter_mut() {
        for aura in character.auras.iter() {
            if aura.target == "self".to_string() {
                if aura.statistic == "health".to_string() {
                    let new_health: i32 = aura.change_health(character.health_max, configs::MAXIMUM_HEALTH);
                    character.health_max = new_health;
                    character.health = new_health;
                } else if aura.statistic == "power".to_string() {
                    let new_power: i32 = aura.change_power(character.power);
                    character.power = new_power;
                } else if aura.statistic == "critical chance".to_string() {
                    let new_crit: f64 = aura.change_crit_chance(character.critical_chance);
                    character.critical_chance = new_crit;
                } else if aura.statistic == "speed".to_string() {
                    let new_speed: i32 = aura.change_speed(character.speed);
                    character.speed = new_speed;
                }
            } else { // collect for deferred initialization
                deferred_auras.push(aura.clone());
            }
        }
    }
    // Loop across deferred auras
    for aura in deferred_auras {
        if aura.target == "allies".to_string() { // handle auras for entire party
            for character in squad.iter_mut() {
                if aura.statistic == "health".to_string() {
                    let new_health: i32 = aura.change_health(character.health_max, configs::MAXIMUM_HEALTH);
                    character.health_max = new_health;
                    character.health = new_health;
                } else if aura.statistic == "power".to_string() {
                    let new_power: i32 = aura.change_power(character.power);
                    character.power = new_power;
                } else if aura.statistic == "critical chance".to_string() {
                    let new_crit: f64 = aura.change_crit_chance(character.critical_chance);
                    character.critical_chance = new_crit;
                } else if aura.statistic == "speed".to_string() {
                    let new_speed: i32 = aura.change_speed(character.speed);
                    character.speed = new_speed;
                }
            }
        }
    }

    return squad;
}

/* --------------------------------------------------------------------------------------------- */

pub fn squad_from_file(filepath: String, directory_characters: &str) -> Vec<Character> {
    let squad_member_names = SquadConstructor::new_from_file(&filepath);
    let mut squad = Vec::with_capacity(5);
    for character_string in squad_member_names.members.iter() {
        let character_path: &str = &format!("{}{}.yml", directory_characters, &character_string);
        squad.push(Character::new_from_file(character_path));
    }
    let squad_output = apply_auras(squad);
    return squad_output;
}

pub fn squad_from_input(directory_characters: &str) -> Vec<Character> {
    let squad_member_names = SquadConstructor::build_from_input();
    let mut squad = Vec::with_capacity(5);
    for character_string in squad_member_names.members.iter() {
        let character_path: &str = &format!("{}{}.yml", directory_characters, &character_string);
        squad.push(Character::new_from_file(character_path));
    }
    let squad_output = apply_auras(squad);
    return squad_output;
}

/* --------------------------------------------------------------------------------------------- */

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     // Aura struct tests
//     #[test]
//     fn test_squad_apply_auras() {
//         let mut test_squad: Vec<Character> = squad::squad_from_file(hero_filepath, &character_folder);
//
//
//
//         assert_eq!(new_value, EXPECTED_VALUE);
//
//     }
// }
