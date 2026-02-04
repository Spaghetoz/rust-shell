//! Functions related with command execution
//! 
//! 

use std::{ffi::CString, ptr};

use libc::{O_APPEND, O_CREAT, O_RDONLY, O_TRUNC, O_WRONLY, S_IRGRP, S_IROTH, S_IRUSR, S_IWUSR, WEXITSTATUS, WIFEXITED, close, dup2, execvp, fork, open, pid_t, pipe, waitpid, write};

use crate::command::{IoFds, RedirectionType, builtin::{change_directory, exit_shell, get_working_directory}};
use crate::command::Command;

impl Command {

    /// Execute the command depending on its type, 
    /// like if it's a simple command, special command (exit, cd...), a pipe, etc...
    /// This function may recursively call the sub commands of &self by passing a transformed iocontext
    /// 
    pub fn execute(&self, io_fds: &IoFds) -> Result<(), Box<dyn std::error::Error>>{
        
        match self {
            Command::Simple{cmd_path, cmd_args} => {

                match cmd_path.as_str() {
                    "exit" => exit_shell(0),
                    // For now, cd takes no more arguments than the path
                    "cd" => change_directory(cmd_args.first().ok_or("cd: missing arg")?)?,
                    "pwd" => {
                        let working_dir = get_working_directory()?;
                        let output = format!("{}\n", working_dir);
                        unsafe {
                            write(io_fds.stdout, output.as_ptr() as *const libc::c_void, output.len());
                        }
                    },
                    _ => execute_simple_command(cmd_path, cmd_args, io_fds)?,
                }
            },
            Command::Redirection { kind, command, file } => {
                execute_redirection_command(kind, command, file, io_fds)?;
            },
            Command::Pipe { left, right } => {
                execute_pipe_command(left, right, io_fds)?;
            }
            
        }
        Ok(())
    }


}

/// Executes a *simple command* by creating a child process
fn execute_simple_command(cmd_path: &str, cmd_args: &[String], io_fds: &IoFds) -> Result<(), Box<dyn std::error::Error>> {  // TODO custom errors types
    
    let mut cmd = std::process::Command::new(cmd_path)
        .args(cmd_args)
        .spawn()?;

        // TODO redirections
    cmd.wait()?; // TODO return value

    Ok(())

}

fn execute_redirection_command(kind: &RedirectionType, command: &Command, file_path: &str, io_fds: &IoFds) -> Result<(), Box<dyn std::error::Error>>  {

    // Select the options creation/read depending on the kind 
    /*let oflag = match kind {
        RedirectionType::In => O_RDONLY,
        RedirectionType::Out | RedirectionType::Err => O_TRUNC | O_CREAT | O_WRONLY,
        RedirectionType::Append => O_WRONLY | O_CREAT | O_APPEND,
    };

    let file_path_cstr = CString::new(file_path)?;
    let file_fd = unsafe { open(file_path_cstr.as_ptr(), oflag, S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH) };

    if file_fd < 0 {
        return Err(format!("Failed to open file {}", file_path).into());
    }
 
    // Creates a new io fds based on the old, but with standards in/out redirected
    let mut new_io_fds = io_fds.clone();
    match kind {
        RedirectionType::In => new_io_fds.stdin = file_fd,
        RedirectionType::Out | RedirectionType::Append => new_io_fds.stdout = file_fd,
        RedirectionType::Err => new_io_fds.stderr = file_fd,
    }
    
    // Recursive call on the command with error checking
    if let Err(err) = command.execute(&new_io_fds) {

        unsafe {close(file_fd); }
        return Err(err);
    }

    unsafe {close(file_fd); }*/

    Ok(())
}

fn execute_pipe_command(left_cmd: &Command, right_cmd: &Command, io_fds: &IoFds) -> Result<(), Box<dyn std::error::Error>> {

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
        left_cmd.execute(io_fds)?;
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
        right_cmd.execute(io_fds)?;
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