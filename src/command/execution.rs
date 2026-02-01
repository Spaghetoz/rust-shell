//! Functions related with command execution
//! 
//! 

use std::{ffi::CString};
use nix::{sys::wait::waitpid, unistd::{ForkResult, fork, execvp}};

use crate::command::{IoContext, builtin::{change_directory, exit_shell, get_working_directory}};
use crate::command::Command;

impl Command {

    /// Execute the command depending on its type, 
    /// like if it's a simple command, special command (exit, cd...), a pipe, etc...
    /// 
    /// * `cmd_io_context` - Where the executed command output should go, 
    ///                      for instance if we the command represents  ls -l | cat , 
    ///                      standard output destination would likely be stdout(),
    ///                      and for a 2> redirection stderr would be a file 
    pub fn execute(&self, cmd_io_context: &mut IoContext) -> Result<(), Box<dyn std::error::Error>>{
        
        match self {
            Command::SimpleCommand{path: cmd_path, args: cmd_args} => {

                match cmd_path.as_str() {
                    "exit" => exit_shell(0),
                    // For now, cd takes no more arguments than the path
                    "cd" => change_directory(&cmd_args.get(0).ok_or("cd: missing arg")?)?,
                    "pwd" => {
                        let working_dir = get_working_directory()?;
                        writeln!(cmd_io_context.stdout, "{working_dir}")?;
                    },
                    _ => execute_simple_command(cmd_path, cmd_args)?,
                }
            },
            
        }
        Ok(())
    }


}

/// Executes a *simple command* by creating a child process
fn execute_simple_command(cmd_path: &str, cmd_args: &[String]) -> Result<(), Box<dyn std::error::Error>> {  // TODO custom errors types

    // Converts cmd and args into the nix lib format
    let cmd = CString::new(cmd_path)?;

    let mut argv: Vec<CString> = Vec::new();
    argv.push(cmd.clone()); // argv[0] = command path

    for arg in cmd_args {
        argv.push(CString::new(arg.as_str())?);
    }

    //TODO dup2 to cmd_context

    unsafe {
        match fork()? {
            ForkResult::Parent { child } => {
                waitpid(child, None)?;
            }
            ForkResult::Child => {
                execvp(&cmd, &argv)?;
            }
        }
    }

    Ok(())

}