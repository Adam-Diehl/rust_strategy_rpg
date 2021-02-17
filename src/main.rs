#![allow(dead_code)]

// Imports => external packages
use colored::*;
use std::env;
use std::path::Path;

// Imports => internal packages
mod configs;
mod character;
mod combat;
mod input;
mod modifiers;
mod targeting;
mod squad;

use character::Character;

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}! [Working title]", "Welcome to <Rust Strategy RPG>".bold());

    let current_path = Path::new("/Users/Presidente/Documents/Programming/Games/hero_battle/data/characters");
    assert!(env::set_current_dir(&current_path).is_ok());

    // let hero_filepath: String = "/Users/Presidente/Documents/Programming/Games/hero_battle/data/squad/test_hero_squad.yml".to_string();
    let villain_filepath: String = "/Users/Presidente/Documents/Programming/Games/hero_battle/data/squad/test_villain_squad.yml".to_string();
    let character_folder = "/Users/Presidente/Documents/Programming/Games/hero_battle/data/characters/";

    let mut villains: Vec<Character> = squad::squad_from_file(villain_filepath, &character_folder);
    println!("You must defeat a squad of evil-doers consisting of the following characters: ");
    for villains in villains.iter() {
        villains.print_pretty_stats();
    }

    println!("You can choose from the following characters:\n - GoodArcherRogue\n - GoodMeleeRogue\n - GoodSwordsman\n - GoodTank");

    // let mut heroes: Vec<Character> = squad::squad_from_file(hero_filepath, &character_folder);
    let mut heroes: Vec<Character> = squad::squad_from_input(&character_folder);
    // for hero in heroes.iter() {
    //     hero.print_pretty_stats();
    // }

    combat::run_combat(&mut heroes, &mut villains);
}
