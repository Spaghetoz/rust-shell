use std::io::{self, Write};

use std::process::Command;

use crate::parsing::tokenize_input;

pub fn run_cli() {

    println!("--- Rust shell ---");

    loop {
        print!("$> ");
        // Flush stdout to directly print without using \n (since stdout is line-buffered)
        io::stdout().flush().expect("stdout flush failed");  // TODO handle error

        let user_input = receive_stdin_input();
        // Turns the input in a vec of Strings 
        let input_tokens = tokenize_input(user_input);

        let command_stdout = execute_command(&input_tokens);
    
        println!("{}", command_stdout);
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

/// Takes a String slice as the command args where command[0] 
/// is the command name and command[1..] its arguments and returns the output on stdout
fn execute_command(command: &[String]) -> String {  // TODO handle error and return a custom stdout/stderr output
    let command_output = Command::new(&command[0]) // TODO handle error
            .args(&command[1..])
            .output()
            .expect("command failed"); // TODO handle error

    let command_stdout = String::from_utf8_lossy(&command_output.stdout);
    
    command_stdout.to_string()
}