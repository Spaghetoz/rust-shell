
//! Module related with commands execution, treatment etc
//! 

use std::process::Command;

use crate::command::builtin::{change_directory, exit_shell, get_working_directory};

mod builtin;

/// Processes the command depending on its type, 
/// like if it's a special command (exit, cd...), a pipe, or another command
pub fn process_command(command: &[&str]) -> Result<(), Box<dyn std::error::Error>>{

    let command_name = command[0];
    
    match command_name { // TODO handle error
        "exit" => exit_shell(0),
        // For now, cd takes no more arguments than the path
        "cd" => change_directory(command[1])?,// TODO handle error index 
        "pwd" => {
            let working_dir = get_working_directory()?;
            println!("{working_dir}");
        },
        _ => {
            let command_stdout = execute_command(command); 
            println!("{}", command_stdout); // TODO move print elsewhere
        }
    }

    Ok(())
}

/// Takes a String slice as the command args where command[0] 
/// is the command name and command[1..] its arguments and returns the output on stdout
pub fn execute_command(command: &[&str]) -> String {  // TODO handle error and return a custom stdout/stderr output
    let command_output = Command::new(&command[0]) // TODO handle error
            .args(&command[1..])
            .output()
            .expect("command failed"); // TODO handle error

    let command_stdout = String::from_utf8_lossy(&command_output.stdout);
    
    command_stdout.to_string()
}