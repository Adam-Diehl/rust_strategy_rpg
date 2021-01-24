use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::character;
use character::Character;
use crate::input;

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

// fn apply_auras()

/* --------------------------------------------------------------------------------------------- */

pub fn squad_from_file(filepath: String, directory_characters: &str) -> Vec<Character> {
    let squad_member_names = SquadConstructor::new_from_file(&filepath);
    let mut squad_output = Vec::with_capacity(5);
    for character_string in squad_member_names.members.iter() {
        let character_path: &str = &format!("{}{}.yml", directory_characters, &character_string);
        squad_output.push(Character::new_from_file(character_path));
    }
    return squad_output;
}

pub fn squad_from_input(directory_characters: &str) -> Vec<Character> {
    let squad_member_names = SquadConstructor::build_from_input();
    let mut squad_output = Vec::with_capacity(5);
    for character_string in squad_member_names.members.iter() {
        let character_path: &str = &format!("{}{}.yml", directory_characters, &character_string);
        squad_output.push(Character::new_from_file(character_path));
    }
    return squad_output;
}
