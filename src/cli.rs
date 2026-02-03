use std::error::Error;
use std::io::{self, Write};

mod interaction;

use crate::cli::interaction::{Interaction, TerminalInteraction, UserInput};
use crate::command::{IoFds};
use crate::parsing::{convert_to_command};
use crate::command::builtin::get_working_directory;

pub fn run_cli() {

    let mut terminal = TerminalInteraction::try_new().expect("error terminal interaction creation");

    println!(" ____            _     ____  _          _ _ ");
    println!("|  _ \\ _   _ ___| |_  / ___|| |__   ___| | |");
    println!("| |_) | | | / __| __| \\___ \\| '_ \\ / _ \\ | |");
    println!("|  _ <| |_| \\__ \\ |_   ___) | | | |  __/ | |");
    println!("|_| \\_\\\\__,_|___/\\__| |____/|_| |_|\\___|_|_|\n");

    loop {
        if let Err(err) = cli_loop_step(&mut terminal) {
            println!("{err}");
        }
    }
}

/// Processes a single step on a loop
pub fn cli_loop_step(terminal: &mut dyn Interaction) -> Result<(), Box<dyn Error>>{

    print_prompt_string();
    println!("");

    let user_input = terminal.receive_input()
        // Propagate the error by specifying it is a user input error
        .map_err(|e| Box::<dyn std::error::Error>::from(format!("Input error: {}", e)))?;

    match user_input {
        UserInput::String(input_string) => {
            
            let input_command = convert_to_command(&input_string)
                .map_err(|e| Box::<dyn std::error::Error>::from(format!("Parsing error: {}", e)))?;
            
            input_command.execute(&IoFds {stdin: 0, stdout: 1, stderr: 2})
                .map_err(|e| Box::<dyn std::error::Error>::from(format!("Execution error: {}", e)))?; 

        },
        UserInput::Interruption => todo!(), 
    }

    Ok(())
}

pub fn print_prompt_string() {
    
    let working_dir = get_working_directory()
        .unwrap_or_else(|_| String::from("unknown"));

    // Prints a pretty colored shell prompt
    print!("$ \x1b[1;34m{}\x1b[0m> ", working_dir);
    
    // Flush stdout to directly print without using \n (since stdout is line-buffered)
    io::stdout().flush().expect("stdout flush failed");  // TODO handle error

}