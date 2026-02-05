//! Functions related with command execution
//! 
//! 

use std::process::{Child, Stdio};
use std::fs::OpenOptions;

use crate::command::{IoContext, RedirectionType, builtin::{change_directory, exit_shell, get_working_directory}};
use crate::command::Command;

impl Command {

    pub fn execute(&self, io_context: IoContext)-> Result<(), Box<dyn std::error::Error>> {

        // Execute the command and waiting the child process if any
        if let Some(mut child_process) = self.execute_recursive(io_context)? {
            child_process.wait()?;
        }

        Ok(())
    }

    /// Execute the command depending on its type,  TODO update comment
    /// like if it's a simple command, special command (exit, cd...), a pipe, etc...
    /// This function may recursively call the sub commands of &self by passing a transformed iocontext
    ///  TODO explain why passing iocontext ownership 
    /// TODO explain return
    fn execute_recursive(&self, io_context: IoContext) -> Result<Option<Child>, Box<dyn std::error::Error>>{
        
        match self {
            Command::Simple{cmd_path, cmd_args} => {

                match cmd_path.as_str() {
                    "exit" => exit_shell(0),
                    // For now, cd takes no more arguments than the path
                    "cd" => change_directory(cmd_args.first().ok_or("cd: missing arg")?)?,
                    "pwd" => {
                        let working_dir = get_working_directory()?;
                        println!("{working_dir}"); // TODO write on io_context.stdout
                    },
                    _ => {
                        return Ok(execute_simple_command(cmd_path, cmd_args, io_context)?); 
                        
                    },
                }
            },
            Command::Redirection { kind, command, file } => {
                return Ok(execute_redirection_command(kind, command, file, io_context)?);
            },
            Command::Pipe { left, right } => {
                return Ok(execute_pipe_command(left, right, io_context)?);
            },
            Command::Separator { left, right } => {
                return Ok(execute_separator_command(left, right, io_context)?);
            }
            
        }

        // No child process, so return None
        Ok(None)
    }


}

/// Executes a *simple command* by creating a child process
fn execute_simple_command(cmd_path: &str, cmd_args: &[String], io_context: IoContext) -> Result<Option<Child>, Box<dyn std::error::Error>> {  // TODO custom errors types

    let cmd = std::process::Command::new(cmd_path)
        .args(cmd_args)
        // If no io context, pass the parent process standard io 
        .stdin(io_context.stdin.unwrap_or(Stdio::inherit()))
        .stdout(io_context.stdout.unwrap_or(Stdio::inherit()))
        .stderr(io_context.stderr.unwrap_or(Stdio::inherit()))
        .spawn()?;


    Ok(Some(cmd))

}

fn execute_redirection_command(kind: &RedirectionType, command: &Command, file_path: &str, io_context: IoContext) -> Result<Option<Child>, Box<dyn std::error::Error>>  {

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
        RedirectionType::In => new_io_context.stdin = Some(Stdio::from(file)),
        RedirectionType::Out | RedirectionType::Append => new_io_context.stdout = Some(Stdio::from(file)),
        RedirectionType::Err => new_io_context.stderr = Some(Stdio::from(file)),
    }
    
    let child_process = command.execute_recursive(new_io_context)?;

    Ok(child_process)
}


fn execute_pipe_command(left_cmd: &Command, right_cmd: &Command, mut io_context: IoContext) -> Result<Option<Child>, Box<dyn std::error::Error>> {

    let new_io_context = IoContext {
        stdin: io_context.stdin.take(),
        stdout: Some(Stdio::piped()),
        stderr: io_context.stderr.take(),
    };

    let left = left_cmd.execute_recursive(new_io_context)?;

    let mut left_child_process = left.ok_or("Missing left child process")?;

    let right_io_context = IoContext {
        stdin: Some(Stdio::from(
            left_child_process.stdout.take().ok_or("take error")?
        )),
        stdout: io_context.stdout.take(),
        stderr: io_context.stderr.take(),
    };

    let mut right_child_process = right_cmd.execute_recursive(right_io_context)?.ok_or("Missing right child")?;

    // Prevent the child from being zombie processes
    right_child_process.wait()?;
    left_child_process.wait()?;

    Ok(Some(right_child_process))
}

fn execute_separator_command(left_cmd: &Command, right_cmd: &Command, io_context: IoContext) -> Result<Option<Child>, Box<dyn std::error::Error>> {

    let mut left = left_cmd.execute_recursive(io_context)?; // TODO dont stop the right cmd execution if the left throws an error
    if let Some(left_child) = &mut left {
        left_child.wait()?;
    }

    let right_io_context = IoContext::new();
    let mut right = right_cmd.execute_recursive(right_io_context)?;

    if let Some(right_child) = &mut right {
        right_child.wait()?;
    }

    Ok(None)
}