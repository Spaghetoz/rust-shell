
//! Module related with commands execution, treatment etc
//! 

use std::process::{Command, Stdio};

use crate::command::builtin::{change_directory, exit_shell, get_working_directory};

pub mod builtin;

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
        _ => execute_command(command),
    }

    Ok(())
}

/// Takes a String slice as the command args where command[0] 
/// is the command name and command[1..] its arguments
pub fn execute_command(command: &[&str]) {  // TODO handle error

    // Child process executing the command
    let mut child = Command::new(&command[0]) // TODO handle error
            .args(&command[1..])
            .stdin(Stdio::inherit())
            .spawn()
            .expect("command failed"); // TODO handle error

    child.wait().expect("wait error"); // TODO handle error

}