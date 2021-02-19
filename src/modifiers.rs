/*
There are two kinds of modifiers: passive auras which are applied before combat begins, and active
abilities which are applied during combat.

File structure:
- Apply trait
- Aura (struct) definition, impl, and Apply impl
- Ability (struct) definition, impl, and Apply impl
- Tests
    - Aura tests
    - Ability tests
*/

use serde::Deserialize;

use crate::configs;

pub trait Apply {
    fn convert_and_add<T: Into<f64> + Copy>(&self, base_value: T) -> f64;
    fn convert_and_multiply<T: Into<f64> + Copy>(&self, base_value: T) -> f64;
    fn change_health(&self, base_value: i32, max_value: i32) -> i32;
    fn change_power(&self, base_value: i32) -> i32;
    fn change_crit_chance(&self, base_value: f64) -> f64;
    fn change_speed(&self, base_value: i32) -> i32;
}

/* --------------------------------------------------------------------------------------------- */

// Used by Character struct
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Aura {
    pub statistic: String,
    pub target: String,
    pub value: f64
}

impl Aura {
    pub fn new(statistic: &str, target: &str, value: f64) -> Aura {
        Aura {statistic: statistic.to_string(), target: target.to_string(), value: value}
    }
}

impl Apply for Aura {
    // Take input (i32 or f64), convert to f64, multiply, then return
    fn convert_and_multiply<T: Into<f64> + Copy>(&self, base_value: T) -> f64 {
        return base_value.into() * (1.0 + self.value)
    }

    // Take input (i32 or f64), convert to f64, add, then return
    fn convert_and_add<T: Into<f64> + Copy>(&self, base_value: T) -> f64 {
        return base_value.into() + self.value
    }

    // Auras don't need a reference to maximum health because they can change it (abilities will need a cap)
    fn change_health(&self, base_value: i32, _max_value: i32) -> i32 {
        return self.convert_and_multiply(base_value).round() as i32
    }

    // Power is uncapped
    fn change_power(&self, base_value: i32) -> i32 {
        return self.convert_and_multiply(base_value).round() as i32
    }

    // Crit chance is capped at 99%
    fn change_crit_chance(&self, base_value: f64) -> f64 {
        let new_crit_chance: f64 = self.convert_and_multiply(base_value);
        return new_crit_chance.min(configs::CRITICAL_CHANCE_CAP)
    }

    // Speed is uncapped
    fn change_speed(&self, base_value: i32) -> i32 {
        return self.convert_and_add(base_value).round() as i32
    }
}

/* --------------------------------------------------------------------------------------------- */

// Can establish a "CheckAbilityTrigger" function and run it at certain pre-determined points,
// like on attack or after death, examples:
//      - CheckAbilityTrigger("Attack", hero, allies, enemies)
//      - CheckAbilityTrigger("Death", hero, allies, enemies)
// Also need to establish a trigger for Abilities -> anything self is easy, allies is harder
//      - Attack & attacked, killed & died

// Used by Character struct
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Ability {
    pub statistic: String,
    pub target: String,
    pub value: f64,
    pub trigger_event: String
}

impl Ability {
    pub fn new(statistic: &str, target: &str, value: f64, event_trigger: &str) -> Ability {
        Ability {statistic: statistic.to_string(), target: target.to_string(), value: value,
            trigger_event: event_trigger.to_string()}
    }

    pub fn check_ability_trigger(&self, event_trigger: &str) -> bool {
        if event_trigger.to_string() == self.trigger_event {
            return true
        } else {
            return false
        }
    }
}

impl Apply for Ability {
    // Take input (i32 or f64), convert to f64, multiply, then return
    fn convert_and_multiply<T: Into<f64> + Copy>(&self, base_value: T) -> f64 {
        return base_value.into() * (1.0 + self.value)
    }

    // Take input (i32 or f64), convert to f64, add, then return
    fn convert_and_add<T: Into<f64> + Copy>(&self, base_value: T) -> f64 {
        return base_value.into() + self.value
    }

    // Health is bounded to starting health to prevent endless battles
    fn change_health(&self, base_value: i32, max_value: i32) -> i32 {
        let new_health: i32 = self.convert_and_multiply(base_value).round() as i32;
        if new_health <= max_value {
            return new_health
        } else {
            return max_value
        }
    }

    // Power is uncapped but bounded below
    fn change_power(&self, base_value: i32) -> i32 {
        let new_power: i32 = self.convert_and_multiply(base_value).round() as i32;
        if new_power < configs::MINIMUM_POWER {
            return configs::MINIMUM_POWER
        } else {
            return new_power
        }
    }

    // Crit chance is capped at 99%
    fn change_crit_chance(&self, base_value: f64) -> f64 {
        let new_crit_chance: f64 = self.convert_and_multiply(base_value);
        return new_crit_chance.min(configs::CRITICAL_CHANCE_CAP)
    }

    // Speed is uncapped
    fn change_speed(&self, base_value: i32) -> i32 {
        return self.convert_and_add(base_value).round() as i32
    }
}

/* --------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    // Aura struct tests
    #[test]
    fn test_aura_convert_and_add_i32() {
        const INPUT_BASE: i32 = 1;
        const VALUE: f64 = 2.0;
        let test_aura = Aura::new("speed", "self", VALUE);
        const EXPECTED_VALUE: f64 = 3.0;

        let new_value = test_aura.convert_and_add(INPUT_BASE);

        assert_eq!(new_value, EXPECTED_VALUE);
    }

    #[test]
    fn test_aura_convert_and_multiply_i32() {
        const INPUT_BASE: i32 = 100;
        const VALUE: f64 = 0.4;
        let test_aura = Aura::new("health", "self", VALUE);
        const EXPECTED_VALUE: f64 = 140.0;

        let new_value = test_aura.convert_and_multiply(INPUT_BASE);

        assert_eq!(new_value, EXPECTED_VALUE);
    }

    #[test]
    fn test_aura_convert_and_multiply_f64() {
        const INPUT_BASE: f64 = 100.0;
        const VALUE: f64 = 0.4;
        let test_aura = Aura::new("health", "self", VALUE);
        const EXPECTED_VALUE: f64 = 140.0;

        let new_value = test_aura.convert_and_multiply(INPUT_BASE);

        assert_eq!(new_value, EXPECTED_VALUE);
    }

    #[test]
    fn test_aura_change_health() {
        const INPUT_BASE: i32 = 100;
        const VALUE: f64 = 0.4;
        let test_aura = Aura::new("health", "self", VALUE);
        const EXPECTED_VALUE: i32 = 140;

        let new_value = test_aura.change_health(INPUT_BASE, configs::MAXIMUM_HEALTH);

        assert_eq!(new_value, EXPECTED_VALUE);
    }

    #[test]
    fn test_aura_change_power() {
        const INPUT_BASE: i32 = 100;
        const VALUE: f64 = 0.4;
        let test_aura = Aura::new("health", "self", VALUE);
        const EXPECTED_VALUE: i32 = 140;

        let new_value = test_aura.change_power(INPUT_BASE);

        assert_eq!(new_value, EXPECTED_VALUE);
    }

    #[test]
    fn test_aura_change_crit_chance() {
        const INPUT_BASE: f64 = 0.25;
        const VALUE: f64 = 0.2;
        let test_aura = Aura::new("critical chance", "self", VALUE);
        const EXPECTED_VALUE: f64 = 0.3;

        let new_value = test_aura.change_crit_chance(INPUT_BASE);

        assert_eq!(new_value, EXPECTED_VALUE);
    }

    #[test]
    fn test_aura_change_speed() {
        const INPUT_BASE: i32 = 0;
        const VALUE: f64 = 2.0;
        let test_aura = Aura::new("speed", "self", VALUE);
        const EXPECTED_VALUE: i32 = 2;

        let new_value = test_aura.change_speed(INPUT_BASE);

        assert_eq!(new_value, EXPECTED_VALUE);
    }
}
