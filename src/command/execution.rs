//! Functions related with command execution
//! 
//! 

use std::{ffi::CString, fs::{OpenOptions, Permissions}, os::{fd::AsFd, unix::fs::PermissionsExt}, sync::Arc};
use nix::{sys::{wait::waitpid}, unistd::{ForkResult, dup2_stdout, execvp, fork, write}};

use crate::command::{IoContext, RedirectionType, builtin::{change_directory, exit_shell, get_working_directory}};
use crate::command::Command;

impl Command {

    /// Execute the command depending on its type, 
    /// like if it's a simple command, special command (exit, cd...), a pipe, etc...
    /// This function may recursively call the sub commands of &self by passing a transformed iocontext
    /// 
    /// * `cmd_io_context` - Where the executed command output should go, 
    ///                      for instance if we the command represents  ls -l | cat , 
    ///                      standard output destination would likely be stdout,
    ///                      and for a 2> redirection stderr would be a file 
    pub fn execute(&self, cmd_io_context: &IoContext) -> Result<(), Box<dyn std::error::Error>>{
        
        match self {
            Command::SimpleCommand{path: cmd_path, args: cmd_args} => {

                match cmd_path.as_str() {
                    "exit" => exit_shell(0),
                    // For now, cd takes no more arguments than the path
                    "cd" => change_directory(&cmd_args.get(0).ok_or("cd: missing arg")?)?,
                    "pwd" => {
                        let working_dir = get_working_directory()?;
                        let output = format!("{}\n", working_dir);
                        write(cmd_io_context.stdout.as_fd(), output.as_bytes())?;
                    },
                    _ => execute_simple_command(cmd_path, cmd_args, cmd_io_context)?,
                }
            },
            Command::Redirection { kind, command, file } => {
                execute_redirection_command(kind, command, file, cmd_io_context)?;
            }
            
        }
        Ok(())
    }


}

/// Executes a *simple command* by creating a child process
fn execute_simple_command(cmd_path: &str, cmd_args: &[String], cmd_io_context: &IoContext) -> Result<(), Box<dyn std::error::Error>> {  // TODO custom errors types

    // Converts cmd and args into the nix lib format
    let cmd = CString::new(cmd_path)?;

    let mut argv: Vec<CString> = Vec::new();
    argv.push(cmd.clone()); // argv[0] = command path

    for arg in cmd_args {
        argv.push(CString::new(arg.as_str())?);
    }

    unsafe {
        match fork()? {
            ForkResult::Parent { child } => {
                waitpid(child, None)?;
            }
            ForkResult::Child => {
                // Redirects the executed command stdout into the context stdout
                dup2_stdout(cmd_io_context.stdout.as_fd())?;

                execvp(&cmd, &argv)?;
            }
        }
    }

    Ok(())

}

fn execute_redirection_command(kind: &RedirectionType, command: &Command, file_path: &str, cmd_io_context: &IoContext) -> Result<(), Box<dyn std::error::Error>>  {
    
    // Select the options creation/read depending on the kind 
    let mut options = OpenOptions::new();
    match kind {
        RedirectionType::Out => {
            options.write(true).create(true).truncate(true);
        },
    }

    let file = options.open(file_path)?;
    let perms = Permissions::from_mode(0o644);
    file.set_permissions(perms)?;

    // Creates a new context based on the old, but with stdout redirected
    let mut new_context = cmd_io_context.clone();
    match kind {
        RedirectionType::Out => new_context.stdout = Arc::new(file)
    }

    // Recursive call on the command
    command.execute(&new_context)?;

    Ok(())
}