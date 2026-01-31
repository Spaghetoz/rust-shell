use std::io::{self, Write};

use crate::parsing::tokenize_input;
use crate::command::process_command;

pub fn run_cli() {

    println!("--- Rust shell ---");

    loop {
        print!("$> ");
        // Flush stdout to directly print without using \n (since stdout is line-buffered)
        io::stdout().flush().expect("stdout flush failed");  // TODO handle error

        let user_input = receive_stdin_input();
        // Turns the input in a vec of Strings 
        let input_tokens = tokenize_input(&user_input);

        if let Err(err) = process_command(&input_tokens) {
            println!("command error {err}");
        }
    
    }
}

fn receive_stdin_input() -> String {

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_string();

    input
    
}