//! Functions related with command execution
//! 
//! 

use std::process::Stdio;
use std::fs::OpenOptions;

use crate::command::{IoContext, RedirectionType, builtin::{change_directory, exit_shell, get_working_directory}};
use crate::command::Command;

impl Command {

    /// Execute the command depending on its type, 
    /// like if it's a simple command, special command (exit, cd...), a pipe, etc...
    /// This function may recursively call the sub commands of &self by passing a transformed iocontext
    ///  TODO explain why passing iocontext ownership 
    pub fn execute(&self, io_context: IoContext) -> Result<(), Box<dyn std::error::Error>>{
        
        match self {
            Command::Simple{cmd_path, cmd_args} => {

                match cmd_path.as_str() {
                    "exit" => exit_shell(0),
                    // For now, cd takes no more arguments than the path
                    "cd" => change_directory(cmd_args.first().ok_or("cd: missing arg")?)?,
                    "pwd" => {
                        // TODO update
                        /*let working_dir = get_working_directory()?;
                        let output = format!("{}\n", working_dir);
                        unsafe {
                            write(io_context.stdout, output.as_ptr() as *const libc::c_void, output.len());
                        }*/
                    },
                    _ => execute_simple_command(cmd_path, cmd_args, io_context)?,
                }
            },
            Command::Redirection { kind, command, file } => {
                execute_redirection_command(kind, command, file, io_context)?;
            },
            Command::Pipe { left, right } => {
                execute_pipe_command(left, right, io_context)?;
            }
            
        }
        Ok(())
    }


}

/// Executes a *simple command* by creating a child process
fn execute_simple_command(cmd_path: &str, cmd_args: &[String], io_context: IoContext) -> Result<(), Box<dyn std::error::Error>> {  // TODO custom errors types

    let mut cmd = std::process::Command::new(cmd_path)
        .args(cmd_args)
        .stdin(io_context.stdin)
        .stdout(io_context.stdout)
        .stderr(io_context.stderr)
        .spawn()?;

    cmd.wait()?; // TODO return value

    Ok(())

}

fn execute_redirection_command(kind: &RedirectionType, command: &Command, file_path: &str, io_context: IoContext) -> Result<(), Box<dyn std::error::Error>>  {

    // Select the options creation/read depending on the kind 
    let mut options = OpenOptions::new();
    match kind {
        RedirectionType::In => {
            options.read(true);
        },
        RedirectionType::Out | RedirectionType::Err => {
            options.truncate(true).create(true).write(true);
        },
        RedirectionType::Append => {
            options.write(true).create(true).append(true);
        },
    }
    let file = options.open(file_path)?;

    let mut new_io_context = IoContext::new(); // TODO use io_context passed in arguments?
    match kind {
        RedirectionType::In => new_io_context.stdin = Stdio::from(file),
        RedirectionType::Out | RedirectionType::Append => new_io_context.stdout = Stdio::from(file),
        RedirectionType::Err => new_io_context.stderr = Stdio::from(file),
    }
    
    command.execute(new_io_context)?;

    Ok(())
}

fn execute_pipe_command(left_cmd: &Command, right_cmd: &Command, io_context: IoContext) -> Result<(), Box<dyn std::error::Error>> {

    /*let mut pipe_fds: [libc::c_int; 2] = [0; 2];
    if unsafe { pipe(pipe_fds.as_mut_ptr())} < 0 {
        return Err("Pipe failed".into());
    }

    // Fork left
    let pid_left = unsafe { fork() };
    if pid_left < 0 {
        return Err("fork error".into());
    } else if pid_left == 0 {
        // child left
        unsafe {
            close(pipe_fds[0]);
            dup2(pipe_fds[1], 1); 
            close(pipe_fds[1]);
        }
        left_cmd.execute(io_context)?;
        std::process::exit(0);
    }

    // Fork right
    let pid_right = unsafe { fork() };
    if pid_right < 0 {
        return Err("fork error".into());
    } else if pid_right == 0 {
        // child right
        unsafe {
            close(pipe_fds[1]); // close write side on the pipe
            dup2(pipe_fds[0], 0); // redirection stdin -> read side
            close(pipe_fds[0]);
        }
        right_cmd.execute(io_context)?;
        std::process::exit(0);
    }

    // Parent
    unsafe {
        close(pipe_fds[0]);
        close(pipe_fds[1]);

        // Wait both children to prevent them to being zombies
        let mut status_left = 0;
        let mut status_right = 0;
        waitpid(pid_left, &mut status_left, 0);
        waitpid(pid_right, &mut status_right, 0);

        if !WIFEXITED(status_left) || WEXITSTATUS(status_left) != 0 {
            return Err("Left command failed".into());
        }
        if !WIFEXITED(status_right) || WEXITSTATUS(status_right) != 0 {
            return Err("Right command failed".into());
        }
    }*/

    Ok(())
}