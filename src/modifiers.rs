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

pub trait Apply {
    fn convert_and_multiply<T: Into<f64> + Copy>(&self, base_value: T) -> f64;
    fn change_health(&self, base_value: i32) -> i32;
    fn change_power(&self, base_value: i32) -> i32;
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

    // Auras don't need a reference to maximum health because they can change it (abilities will need a cap)
    fn change_health(&self, base_value: i32) -> i32 {
        return self.convert_and_multiply(base_value).round() as i32
    }

    // Power is uncapped
    fn change_power(&self, base_value: i32) -> i32 {
        return self.convert_and_multiply(base_value).round() as i32
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
    statistic: String,
    target: String,
    value: f64
}

impl Ability {
    pub fn new(statistic: &str, target: &str, value: f64) -> Ability {
        Ability {statistic: statistic.to_string(), target: target.to_string(), value: value}
    }
}

// impl Apply for Ability {
//
// }

/* --------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    // Aura struct tests
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

        let new_value = test_aura.change_health(INPUT_BASE);

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

}
