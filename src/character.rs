use std::cmp;
use colored::*;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::configs;
use crate::modifiers;
use modifiers::Aura;
use modifiers::Ability;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Character {
    pub name: String,
    pub class: String,
    pub speed: i32,
    pub health: i32,
    pub health_max: i32,
    pub power: i32,
    pub critical_chance: f64,
    pub attack_type: String,
    pub dt: i32, // damage threshold (flat amount of damage ignored)
    pub dr: f64, // damage reduction (percent taken off the top)
    pub tags: Vec<String>, // modifiers/properties characters can possess
    pub auras: Vec<Aura>,
    pub abilities: Vec<Ability>,
    xp: i32,
    pub level: u32,
    pub description: String
}

// Read a character from file
pub fn read_params_from_file<P: AsRef<Path>>(path: P) -> Result<Character, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let params = serde_yaml::from_reader(reader)?;
    Ok(params)
}

impl Character {
    #[allow(dead_code)] // Used by tests
    pub fn new(
        name: &str,
        class: &str,
        speed: i32,
        health: i32,
        power: i32,
        critical_chance: f64,
        attack_type: &str,
        dt: i32,
        dr: f64,
        tags: Vec<String>,
        auras: Vec<Aura>,
        abilities: Vec<Ability>
    ) -> Character {
        Character {
            name: name.to_string(),
            class: class.to_string(),
            speed: speed,
            health: health,
            health_max: health,
            power: power,
            critical_chance: critical_chance,
            attack_type: attack_type.to_string(),
            dt: dt,
            dr: dr,
            tags: tags,
            auras: auras,
            abilities: abilities,
            xp: 0,
            level: 1,
            description: "null".to_string()
        }
    }

    pub fn new_from_file(filepath: &str) -> Self {
        let mut character: Character = read_params_from_file(filepath).unwrap();
        Character::validate_actor_data(&mut character);
        return character;
    }

    // Validation
    fn validate_actor_data(&mut self) {
        if self.health > self.health_max {
            println!("WARNING: data error in {}: current health ({}) > maximum health ({}). Overriding automatically.", self.name, self.health, self.health_max);
            self.health = self.health_max;
        }
    }

    // Combat helper functions

    // Damage calculations (accounting for armor) => crit variable is passed through to print damage taken fn
    pub fn take_damage(&mut self, power: i32, crit: bool) {
        // Cap damage resistance at 80%
        let effective_dr: f64 = self.dr.min(configs::MAXIUMUM_DAMAGE_RESIST);
        // Apply damage reduction
        let reduced_damage: i32 = ((1.0 - effective_dr) * power as f64).round() as i32;
        // Apply damage threshold
        let actual_damage: i32 = std::cmp::max(reduced_damage - self.dt, configs::MINIMUM_DAMAGE);
        self.health -= actual_damage;
        self.print_damage_taken(actual_damage, crit);
    }

    // Check if character died
    pub fn is_dead(&self) -> bool {
        if self.health <= 0 {
            return true;
        } else {
            return false;
        }
    }

    // Levelling
    #[allow(dead_code)]
    pub fn add_xp(&mut self, xp_gained: i32) {
        self.xp += xp_gained;
        if self.xp > 10 {
            self.level += 1;
        }
    }

    // IO
    #[allow(dead_code)]
    pub fn print_debug_stats(&self) {
        println!("Current Statistics: {}", self.name);
        println!("{:?}", self);
    }

    pub fn print_pretty_stats(&self) {
        println!("--------------------");
        println!("| Name: {}", self.name);
        println!("| Health: {}", self.health);
        println!("| Power: {}", self.power);
        println!("| Crit. Chance: {}%", 100.0 * self.critical_chance);
        println!("{}", self.description);
        println!("--------------------\n");
    }

    // IO => Combat IO
    pub fn print_attacking(&self, target_name: &str) {
        // No newline b/c chained with print_damage_taken function
        print!("{} is attacking {}! ", self.name, target_name);
    }

    fn print_damage_taken(&self, damage: i32, is_crit: bool) {
        let health_rounded: i32 = cmp::max(self.health, 0);
        let health_percent: i32 = (100.0 * (health_rounded as f64 / self.health_max as f64)).round() as i32;
        if is_crit {
            if health_percent >= configs::HEALTH_LEVEL_GREEN {
                println!("{}! {} took {} points of damage ({}% HP remaining).", "Critical hit".to_string().bold(), self.name, damage.to_string().bold(), health_percent.to_string().green());
            } else if health_percent >= configs::HEALTH_LEVEL_YELLOW {
                println!("{}! {} took {} points of damage ({}% HP remaining).", "Critical hit".to_string().bold(), self.name, damage.to_string().bold(), health_percent.to_string().yellow());
            } else {
                println!("{}! {} took {} points of damage ({}% HP remaining).", "Critical hit".to_string().bold(), self.name, damage.to_string().bold(), health_percent.to_string().red());
            }
        } else {
            if health_percent >= configs::HEALTH_LEVEL_GREEN {
                println!("{} took {} points of damage ({}% HP remaining).", self.name, damage.to_string(), health_percent.to_string().green());
            } else if health_percent >= configs::HEALTH_LEVEL_YELLOW {
                println!("{} took {} points of damage ({}% HP remaining).", self.name, damage.to_string(), health_percent.to_string().yellow());
            } else {
                println!("{} took {} points of damage ({}% HP remaining).", self.name, damage.to_string(), health_percent.to_string().red());
            }
        }
    }

    pub fn print_died(&self) {
        let output = format!("{} died!", self.name);
        println!("{}", output.red());
    }

}

/* --------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    // Character Struct tests
    #[test]
    fn test_new() {
        const EXPECTED_NAME: &str = "Dave";
        const EXPECTED_CLASS: &str = "Warrior";
        const EXPECTED_SPEED: i32 = 1;
        const EXPECTED_HEALTH: i32 = 10;
        const EXPECTED_POWER: i32 = 5;
        const EXPECTED_CRIT: f64 = 0.25;
        let expected_attack_type: String = "single".to_string();
        const EXPECTED_DT: i32 = 0;
        const EXPECTED_DR: f64 = 0.0;
        let input_tags: Vec<String> = vec!["null".to_string()];
        let input_auras: Vec<Aura> = vec![Aura::new("null", "null", 0.0)];
        let input_abilities: Vec<Ability> = vec![Ability::new("null", "null", 0.0)];
        let expected_tags: Vec<String> = vec!["null".to_string()];
        let expected_auras: Vec<Aura> = vec![Aura::new("null", "null", 0.0)];
        let expected_abilities: Vec<Ability> = vec![Ability::new("null", "null", 0.0)];

        let test_character = Character::new(EXPECTED_NAME, EXPECTED_CLASS, EXPECTED_SPEED,
            EXPECTED_HEALTH, EXPECTED_POWER, EXPECTED_CRIT, &expected_attack_type, EXPECTED_DT,
            EXPECTED_DR, input_tags, input_auras, input_abilities);

        assert_eq!(test_character.name, EXPECTED_NAME);
        assert_eq!(test_character.class, EXPECTED_CLASS);
        assert_eq!(test_character.speed, EXPECTED_SPEED);
        assert_eq!(test_character.health, EXPECTED_HEALTH);
        assert_eq!(test_character.power, EXPECTED_POWER);
        assert_eq!(test_character.critical_chance, EXPECTED_CRIT);
        assert_eq!(test_character.attack_type, expected_attack_type);
        assert_eq!(test_character.tags, expected_tags);
        assert_eq!(test_character.auras, expected_auras);
        assert_eq!(test_character.abilities, expected_abilities);
    }

    #[test]
    fn test_take_damage_no_armor() {
        const NAME: &str = "Dave";
        const CLASS: &str = "Warrior";
        const SPEED: i32 = 1;
        const HEALTH: i32 = 20;
        const POWER: i32 = 5;
        const CRIT: bool = false;
        const CRITICAL_CHANCE: f64 = 0.0;
        let attack_type: String = "single".to_string();
        const DT: i32 = 0;
        const DR: f64 = 0.0;
        let tags: Vec<String> = vec!["null".to_string()];
        let auras: Vec<Aura> = vec![Aura::new("null", "null", 0.0)];
        let abilities: Vec<Ability> = vec![Ability::new("null", "null", 0.0)];
        const INCOMING_DAMAGE: i32 = 10;
        const EXPECTED_HEALTH: i32 = 10;

        let mut test_character = Character::new(NAME, CLASS, SPEED, HEALTH, POWER, CRITICAL_CHANCE,
            &attack_type, DT, DR, tags, auras, abilities);
        test_character.take_damage(INCOMING_DAMAGE, CRIT);
        assert_eq!(test_character.health, EXPECTED_HEALTH);
    }

    #[test]
    fn test_take_damage_dt_only() {
        const NAME: &str = "Dave";
        const CLASS: &str = "Warrior";
        const SPEED: i32 = 1;
        const HEALTH: i32 = 20;
        const POWER: i32 = 5;
        const CRIT: bool = false;
        const CRITICAL_CHANCE: f64 = 0.0;
        let attack_type: String = "single".to_string();
        const DT: i32 = 1;
        const DR: f64 = 0.0;
        let tags: Vec<String> = vec!["null".to_string()];
        let auras: Vec<Aura> = vec![Aura::new("null", "null", 0.0)];
        let abilities: Vec<Ability> = vec![Ability::new("null", "null", 0.0)];
        const INCOMING_DAMAGE: i32 = 15;
        const EXPECTED_HEALTH: i32 = 6;

        let mut test_character = Character::new(NAME, CLASS, SPEED, HEALTH, POWER, CRITICAL_CHANCE,
            &attack_type, DT, DR, tags, auras, abilities);
        test_character.take_damage(INCOMING_DAMAGE, CRIT);
        assert_eq!(test_character.health, EXPECTED_HEALTH);
    }

    #[test]
    fn test_take_damage_dr_only() {
        const NAME: &str = "Dave";
        const CLASS: &str = "Warrior";
        const SPEED: i32 = 1;
        const HEALTH: i32 = 100;
        const POWER: i32 = 5;
        const CRIT: bool = false;
        const CRITICAL_CHANCE: f64 = 0.0;
        let attack_type: String = "single".to_string();
        const DT: i32 = 0;
        const DR: f64 = 0.2;
        let tags: Vec<String> = vec!["null".to_string()];
        let auras: Vec<Aura> = vec![Aura::new("null", "null", 0.0)];
        let abilities: Vec<Ability> = vec![Ability::new("null", "null", 0.0)];
        const INCOMING_DAMAGE: i32 = 50;
        const EXPECTED_HEALTH: i32 = 60;

        let mut test_character = Character::new(NAME, CLASS, SPEED, HEALTH, POWER, CRITICAL_CHANCE,
            &attack_type, DT, DR, tags, auras, abilities);
        test_character.take_damage(INCOMING_DAMAGE, CRIT);
        assert_eq!(test_character.health, EXPECTED_HEALTH);
    }

    #[test]
    fn test_take_damage_dt_and_dr() {
        const NAME: &str = "Dave";
        const CLASS: &str = "Warrior";
        const SPEED: i32 = 1;
        const HEALTH: i32 = 100;
        const POWER: i32 = 5;
        const CRIT: bool = false;
        const CRITICAL_CHANCE: f64 = 0.0;
        let attack_type: String = "single".to_string();
        const DT: i32 = 1;
        const DR: f64 = 0.2;
        let tags: Vec<String> = vec!["null".to_string()];
        let auras: Vec<Aura> = vec![Aura::new("null", "null", 0.0)];
        let abilities: Vec<Ability> = vec![Ability::new("null", "null", 0.0)];
        const INCOMING_DAMAGE: i32 = 50;
        const EXPECTED_HEALTH: i32 = 61;

        let mut test_character = Character::new(NAME, CLASS, SPEED, HEALTH, POWER, CRITICAL_CHANCE,
            &attack_type, DT, DR, tags, auras, abilities);
        test_character.take_damage(INCOMING_DAMAGE, CRIT);
        assert_eq!(test_character.health, EXPECTED_HEALTH);
    }

    #[test]
    fn test_is_dead() {
        const NAME: &str = "Dave";
        const CLASS: &str = "Warrior";
        const SPEED: i32 = 1;
        const HEALTH: i32 = 10;
        const POWER: i32 = 5;
        const CRIT: bool = false;
        const CRITICAL_CHANCE: f64 = 0.0;
        let attack_type: String = "single".to_string();
        const DT: i32 = 0;
        const DR: f64 = 0.0;
        let tags: Vec<String> = vec!["null".to_string()];
        let auras: Vec<Aura> = vec![Aura::new("null", "null", 0.0)];
        let abilities: Vec<Ability> = vec![Ability::new("null", "null", 0.0)];
        const INCOMING_DAMAGE: i32 = 40;
        const EXPECTED_DEATH: bool = true;

        let mut test_character = Character::new(NAME, CLASS, SPEED, HEALTH, POWER, CRITICAL_CHANCE,
            &attack_type, DT, DR, tags, auras, abilities);
        test_character.take_damage(INCOMING_DAMAGE, CRIT);
        let death_result = test_character.is_dead();

        assert_eq!(death_result, EXPECTED_DEATH);
    }
}
