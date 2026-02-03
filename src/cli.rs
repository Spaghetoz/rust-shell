use std::io::{self, Write};

mod interaction;

use crate::cli::interaction::{Interaction, TerminalInteraction};
use crate::command::{IoFds};
use crate::parsing::{convert_to_command};
use crate::command::builtin::get_working_directory;

pub fn run_cli() {

    let terminal_fds = IoFds {
        stdin: 0,
        stdout: 1,
        stderr: 2,
    };
    let terminal = TerminalInteraction::new();

    println!(" ____            _     ____  _          _ _ ");
    println!("|  _ \\ _   _ ___| |_  / ___|| |__   ___| | |");
    println!("| |_) | | | / __| __| \\___ \\| '_ \\ / _ \\ | |");
    println!("|  _ <| |_| \\__ \\ |_   ___) | | | |  __/ | |");
    println!("|_| \\_\\\\__,_|___/\\__| |____/|_| |_|\\___|_|_|\n");

    loop {

        let working_dir = get_working_directory().expect("get_working_directory failed"); // TODO handle error
        // Prints a pretty colored shell prompt
        print!("$ \x1b[1;34m{}\x1b[0m> ", working_dir);
        // Flush stdout to directly print without using \n (since stdout is line-buffered)
        io::stdout().flush().expect("stdout flush failed");  // TODO handle error

        let user_input = match terminal.receive_input() {
            Ok(input) => input,
            Err(err) => {
                println!("input error: {err}");
                continue;
            }
        };

        let input_command = match convert_to_command(&user_input) {
            Ok(command) => command,
            Err(err) => {
                println!("Parsing error : {err}");
                continue;
            }
        };
        
        if let Err(err) = input_command.execute(&terminal_fds) {
            println!("{err}");
        }
    
    }
}
