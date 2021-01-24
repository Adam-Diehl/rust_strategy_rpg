/*
Targeting:
    - Characters stored in one of five slots/positions
    - Assumed positions is a square (4 characters - size of standard RPG party) + pet, with positions set in configs
    - Front row tanks / gets attacked first barring special circumstances (flankers, AOE, etc.)
    - Attack types
        - Single attacker
        - Row attacker
        - Column attacker
        - All attacker
*/

use crate::configs;

pub fn attack_type_to_coordinates(attack_type: &str, alive_targets: Vec<bool>, attacker_tags: &Vec<String>) -> Vec<bool> {
    let num_targets = alive_targets.len();
    let mut targets: Vec<bool> = Vec::with_capacity(configs::TEAM_SIZE);
    if attack_type == "row" {
        // Initialize variables to fill inside of if/else statement
        let pos_left_primary: usize;
        let pos_right_primary: usize;
        let pos_left_backup: usize;
        let pos_right_backup: usize;
        // flankers target back row first
        if attacker_tags.contains(&"flanker".to_string()) {
            pos_left_primary = configs::BACK_LEFT;
            pos_right_primary = configs::BACK_RIGHT;
            pos_left_backup = configs::FRONT_LEFT;
            pos_right_backup = configs::FRONT_RIGHT;
        } else { // everyone else targets front row first
            pos_left_primary = configs::FRONT_LEFT;
            pos_right_primary = configs::FRONT_RIGHT;
            pos_left_backup = configs::BACK_LEFT;
            pos_right_backup = configs::BACK_RIGHT;
        }
        // Loop across targets to assign attacks
        for i in 0..num_targets {
            if alive_targets[pos_left_primary] || alive_targets[pos_right_primary] {
                if alive_targets[i] && (i == pos_left_primary || i == pos_right_primary) {
                    targets.push(true);
                } else {
                    targets.push(false);
                }
            } else {
                if alive_targets[i] && (i == pos_left_backup || i == pos_right_backup) {
                    targets.push(true);
                } else {
                    targets.push(false);
                }
            }
        }
        return targets;
    } else if attack_type == "column" {
        // Attacks left column by default -- TODO -> update to use rand to 50/50 left and right
        for i in 0..num_targets {
            if alive_targets[configs::FRONT_LEFT] || alive_targets[configs::BACK_LEFT] { // target left column if alive
                if alive_targets[i] && (i == configs::FRONT_LEFT || i == configs::BACK_LEFT) { // if alive and in left column
                    targets.push(true);
                } else {
                    targets.push(false);
                }
            } else { // target back row
                if alive_targets[i] && (i == configs::FRONT_RIGHT || i == configs::BACK_RIGHT) { // if alive and in right column
                    targets.push(true);
                } else {
                    targets.push(false);
                }
            }
        }
        return targets;
    } else if attack_type == "all" {
        for i in 0..num_targets {
            if alive_targets[i] { // if alive, then attack
                targets.push(true);
            } else {
                targets.push(false);
            }
        }
        return targets;
    } else { // Anything else gets thrown to single attacker type
        // Attacks front left by default -- TODO -> update to use rand to 50/50 left and right
        let mut target_not_selected: bool = true; // once a single target has been chosen will change to true
        if attacker_tags.contains(&"flanker".to_string()) { // flankers target back row first
            for i in (0..num_targets).rev() {
                if alive_targets[i] && target_not_selected {
                    targets.push(true);
                    target_not_selected = false;
                } else {
                    targets.push(false);
                }
            }
            targets.reverse();
            return targets;
        } else { // normal single attackers target front row first
            for i in 0..num_targets {
                if alive_targets[i] && target_not_selected {
                    targets.push(true);
                    target_not_selected = false;
                } else {
                    targets.push(false);
                }
            }
            return targets;
        }
    }
}

/* --------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    // Target generation tests

    // Single target -> front row
    #[test]
    fn test_attack_to_coordinates_single_front() {
        let attack_type: &str = "single";
        let alive_targets: Vec<bool> = vec![true, true, false, true];
        let expected_targets: Vec<bool> = vec![true, false, false, false];
        let tags: Vec<String> = vec!["null".to_string()];

        let targets: Vec<bool> = attack_type_to_coordinates(attack_type, alive_targets, &tags);
        assert_eq!(targets, expected_targets);
    }

    // Single target -> back row
    #[test]
    fn test_attack_to_coordinates_single_back() {
        let attack_type: &str = "single";
        let alive_targets: Vec<bool> = vec![false, false, false, true];
        let expected_targets: Vec<bool> = vec![false, false, false, true];
        let tags: Vec<String> = vec!["null".to_string()];

        let targets: Vec<bool> = attack_type_to_coordinates(attack_type, alive_targets, &tags);
        assert_eq!(targets, expected_targets);
    }

    // Single target -> flanker
    #[test]
    fn test_attack_to_coordinates_single_flanker() {
        let attack_type: &str = "single";
        let alive_targets: Vec<bool> = vec![true, false, true, false];
        let expected_targets: Vec<bool> = vec![false, false, true, false];
        let tags: Vec<String> = vec!["flanker".to_string()];

        let targets: Vec<bool> = attack_type_to_coordinates(attack_type, alive_targets, &tags);
        assert_eq!(targets, expected_targets);
    }

    // Test front row when full row alive (not flanker)
    #[test]
    fn test_attack_to_coordinates_row_front() {
        let attack_type: &str = "row";
        let alive_targets: Vec<bool> = vec![true, true, false, true];
        let expected_targets: Vec<bool> = vec![true, true, false, false];
        let tags: Vec<String> = vec!["null".to_string()];

        let targets: Vec<bool> = attack_type_to_coordinates(attack_type, alive_targets, &tags);
        assert_eq!(targets, expected_targets);
    }

    // Test back row when full row alive (not lanker)
    #[test]
    fn test_attack_to_coordinates_row_back() {
        let attack_type: &str = "row";
        let alive_targets: Vec<bool> = vec![false, false, true, true];
        let expected_targets: Vec<bool> = vec![false, false, true, true];
        let tags: Vec<String> = vec!["null".to_string()];

        let targets: Vec<bool> = attack_type_to_coordinates(attack_type, alive_targets, &tags);
        assert_eq!(targets, expected_targets);
    }

    // Test front row when row not fully alive (not lanker)
    #[test]
    fn test_attack_to_coordinates_row_front_partial() {
        let attack_type: &str = "row";
        let alive_targets: Vec<bool> = vec![false, true, false, true];
        let expected_targets: Vec<bool> = vec![false, true, false, false];
        let tags: Vec<String> = vec!["null".to_string()];

        let targets: Vec<bool> = attack_type_to_coordinates(attack_type, alive_targets, &tags);
        assert_eq!(targets, expected_targets);
    }

    // Test back row when row not fully alive (not lanker)
    #[test]
    fn test_attack_to_coordinates_row_back_partial() {
        let attack_type: &str = "row";
        let alive_targets: Vec<bool> = vec![false, false, true, false];
        let expected_targets: Vec<bool> = vec![false, false, true, false];
        let tags: Vec<String> = vec!["null".to_string()];

        let targets: Vec<bool> = attack_type_to_coordinates(attack_type, alive_targets, &tags);
        assert_eq!(targets, expected_targets);
    }

    // Test front row when full row alive (flanker)
    #[test]
    fn test_attack_to_coordinates_row_back_flanker() {
        let attack_type: &str = "row";
        let alive_targets: Vec<bool> = vec![true, false, true, true];
        let expected_targets: Vec<bool> = vec![false, false, true, true];
        let tags: Vec<String> = vec!["flanker".to_string()];

        let targets: Vec<bool> = attack_type_to_coordinates(attack_type, alive_targets, &tags);
        assert_eq!(targets, expected_targets);
    }

    // Test left column
    #[test]
    fn test_attack_to_coordinates_left_column() {
        let attack_type: &str = "column";
        let alive_targets: Vec<bool> = vec![true, true, true, false];
        let expected_targets: Vec<bool> = vec![true, false, true, false];
        let tags: Vec<String> = vec!["null".to_string()];

        let targets: Vec<bool> = attack_type_to_coordinates(attack_type, alive_targets, &tags);
        assert_eq!(targets, expected_targets);
    }

    // Test right column
    #[test]
    fn test_attack_to_coordinates_right_column() {
        let attack_type: &str = "column";
        let alive_targets: Vec<bool> = vec![false, true, false, true];
        let expected_targets: Vec<bool> = vec![false, true, false, true];
        let tags: Vec<String> = vec!["null".to_string()];

        let targets: Vec<bool> = attack_type_to_coordinates(attack_type, alive_targets, &tags);
        assert_eq!(targets, expected_targets);
    }

    // Test all
    #[test]
    fn test_attack_to_coordinates_all() {
        let attack_type: &str = "all";
        let alive_targets: Vec<bool> = vec![true, true, false, true];
        let expected_targets: Vec<bool> = vec![true, true, false, true];
        let tags: Vec<String> = vec!["null".to_string()];

        let targets: Vec<bool> = attack_type_to_coordinates(attack_type, alive_targets, &tags);
        assert_eq!(targets, expected_targets);
    }
}
