/*
File to set global parameters (that are constant).
*/

// Combat parameters
pub const CRITICAL_CHANCE_CAP: f64 = 0.99;
pub const CRITICAL_MULTIPLIER: i32 = 2;
pub const MINIMUM_DAMAGE: i32 = 10;
pub const MAXIUMUM_DAMAGE_RESIST: f64 = 0.85;

// Combat IO parameters
pub const HEALTH_LEVEL_GREEN: i32 = 67;
pub const HEALTH_LEVEL_YELLOW: i32 = 33;

// Positioning parameters
pub const TEAM_SIZE: usize = 5;
pub const FRONT_LEFT: usize = 0;
pub const FRONT_RIGHT: usize = 1;
pub const BACK_LEFT: usize = 2;
pub const BACK_RIGHT: usize = 3;
#[allow(dead_code)]
pub const PET: usize = 4;
