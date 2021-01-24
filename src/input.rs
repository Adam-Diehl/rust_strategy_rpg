use std::io;

// Grab user input
pub fn grab_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let command = input.trim().to_string();
    return command;
}
