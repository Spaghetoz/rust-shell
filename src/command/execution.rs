//! Functions related with command execution
//! 
//! 

use std::ffi::CString;
use nix::{sys::wait::waitpid, unistd::{ForkResult, fork, execvp}};

use crate::command::builtin::{change_directory, exit_shell, get_working_directory};
use crate::command::Command;

impl Command {

    /// Execute the command depending on its type, 
    /// like if it's a simple command, special command (exit, cd...), a pipe, etc...
    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>>{
    
        let Command::SimpleCommand{path: cmd_path, args: cmd_args} = self;
        
        match cmd_path.as_str() { // TODO handle error
            "exit" => exit_shell(0),
            // For now, cd takes no more arguments than the path
            "cd" => change_directory(&cmd_args[0])?,// TODO handle error index 
            "pwd" => {
                let working_dir = get_working_directory()?;
                println!("{working_dir}");
            },
            _ => self.execute_simple()?,
        }

        Ok(())
    }

    /// Executes the command by creating a child process
    fn execute_simple(&self) -> Result<(), Box<dyn std::error::Error>> {  // TODO handle error

        // TODO handle other commands types
        let Command::SimpleCommand{path: cmd_path, args: cmd_args} = self;

        // Converts cmd and args into the nix lib format
        let cmd = CString::new(cmd_path.clone())?; // TODO find solution without cloning?
        let args: Vec<CString> = cmd_args
            .iter()
            .map(|s| CString::new(s.as_str()).unwrap()) // TODO fix unwrap
            .collect();

        unsafe {
            match fork()?  {
                ForkResult::Parent { child } => {
                    // Prevent zombie processes
                    waitpid(child, None)?;
                }
                ForkResult::Child => {
                    execvp(&cmd, &args)?;
                }
            }
        }

        Ok(())

    }

}
