//! Functions related with command execution
//! 
//! 

use std::{ffi::CString, ptr};

use libc::{WEXITSTATUS, WIFEXITED, dup2, execvp, fork, pid_t, waitpid, write};

use crate::command::{IoFds, RedirectionType, builtin::{change_directory, exit_shell, get_working_directory}};
use crate::command::Command;

impl Command {

    /// Execute the command depending on its type, 
    /// like if it's a simple command, special command (exit, cd...), a pipe, etc...
    /// This function may recursively call the sub commands of &self by passing a transformed iocontext
    /// 
    pub fn execute(&self, io_fds: &IoFds) -> Result<(), Box<dyn std::error::Error>>{
        
        match self {
            Command::SimpleCommand{path: cmd_path, args: cmd_args} => {

                match cmd_path.as_str() {
                    "exit" => exit_shell(0),
                    // For now, cd takes no more arguments than the path
                    "cd" => change_directory(&cmd_args.get(0).ok_or("cd: missing arg")?)?,
                    "pwd" => {
                        let working_dir = get_working_directory()?;

                        unsafe {
                            if write(io_fds.stdout, working_dir.as_ptr() as *const _, working_dir.len()) < 0 {
                                return Err("Write failed".into());
                            }
                        }
                    },
                    _ => execute_simple_command(cmd_path, cmd_args, io_fds)?,
                }
            },
            Command::Redirection { kind, command, file } => {
                execute_redirection_command(kind, command, file, io_fds)?;
            }
            
        }
        Ok(())
    }


}

/// Executes a *simple command* by creating a child process
fn execute_simple_command(cmd_path: &str, cmd_args: &[String], io_fds: &IoFds) -> Result<(), Box<dyn std::error::Error>> {  // TODO custom errors types
    
    // Converts cmd and args into the libc format
    let cmd = CString::new(cmd_path)?;
    let mut cstrings: Vec<CString> = vec![cmd.clone()];  // argv[0] = command path

    for arg in cmd_args {
        cstrings.push(CString::new(arg.as_str())?);
    }

    let mut argv: Vec<*const libc::c_char> = cstrings.iter().map(|c| c.as_ptr()).collect();
    argv.push(ptr::null());

    unsafe {

        let pid : pid_t = fork();
        if pid < 0 {
            return Err("fork error".into());
        } 
        else if pid == 0 {

            if dup2(io_fds.stdin, 0) < 0 || dup2(io_fds.stdout, 1) < 0 || dup2(io_fds.stderr, 2) < 0 {
                eprintln!("dup2 failed");
                std::process::exit(1);
            }

            execvp(cmd.as_ptr(), argv.as_ptr());
            std::process::exit(1);
            
        } else {

            let mut status: i32 = 0;
            waitpid(pid, &mut status,0);
                        
            if !WIFEXITED(status) || WEXITSTATUS(status) != 0 {
                return Err(format!("child process exited with status {}", libc::WEXITSTATUS(status)).into());
            }
        }

    }

    Ok(())

}

fn execute_redirection_command(kind: &RedirectionType, command: &Command, file_path: &str, io_fds: &IoFds) -> Result<(), Box<dyn std::error::Error>>  {
    
    // Select the options creation/read depending on the kind 
    /*let mut options = OpenOptions::new();
    match kind {
        RedirectionType::In => { options.read(true); },
        RedirectionType::Out | RedirectionType::Err =>  { options.write(true).create(true).truncate(true); },
        RedirectionType::Append => { options.write(true).create(true).append(true); },
    }       

    let file = options.open(file_path)?;
    
    if !matches!(kind, RedirectionType::In) {
        let perms = Permissions::from_mode(0o644);
        file.set_permissions(perms)?;
    }


    // Creates a new context based on the old, but with standards in/out redirected
    let mut new_context = cmd_io_context.clone();
    match kind {
        RedirectionType::In => new_context.stdin = Arc::new(file),
        RedirectionType::Out | RedirectionType::Append => new_context.stdout = Arc::new(file),
        RedirectionType::Err => new_context.stderr = Arc::new(file),
    }

    // Recursive call on the command
    command.execute(&new_context)?;*/

    Ok(())
}

