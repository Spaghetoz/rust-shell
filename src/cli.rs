use std::io::{self, Write};

use std::fs::File;
use std::os::unix::io::FromRawFd;

use crate::command::IoContext;
use crate::parsing::{convert_to_command};
use crate::command::builtin::get_working_directory;

pub fn run_cli() {

    // Variable containing what stdin, stdout and stderr should be for the terminal
    let mut terminal_io_context = IoContext {    
        stdin: unsafe { File::from_raw_fd(0) },   
        stdout: unsafe { File::from_raw_fd(1) },  
        stderr: unsafe { File::from_raw_fd(2) },
    };

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

        let user_input = receive_stdin_input();
        let input_command = convert_to_command(&user_input); 

        if let Err(err) = input_command.execute(&mut terminal_io_context) {
            println!("{err}");
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