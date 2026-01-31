
//! Module related with commands execution, treatment etc
//! 

use std::process::{Stdio};

use crate::command::builtin::{change_directory, exit_shell, get_working_directory};

pub mod builtin;


pub enum Command {
    SimpleCommand {
        path: String,
        args: Vec<String>,
    },
    /*Pipe {
        left: Box<Command>,
        right: Box<Command>,
    },
    Redirection {
        kind: RedirectionType,
        command: Box<Command>,
        file: String,
    },*/
}

/*pub enum RedirectionType {
    In,       // <
    Out,      // >
    Append,   // >>
    Err,      // 2>
}*/

impl Command {

    /// Processes the command depending on its type, 
    /// like if it's a special command (exit, cd...), a pipe, or another command
    pub fn process_command(&self) -> Result<(), Box<dyn std::error::Error>>{
    
        let Command::SimpleCommand{path: cmd_path, args: cmd_args} = self;
        
        match cmd_path.as_str() { // TODO handle error
            "exit" => exit_shell(0),
            // For now, cd takes no more arguments than the path
            "cd" => change_directory(&cmd_args[0])?,// TODO handle error index 
            "pwd" => {
                let working_dir = get_working_directory()?;
                println!("{working_dir}");
            },
            _ => self.execute(),
        }

        Ok(())
    }

    /// Executes the command by creating a child process
    pub fn execute(&self) {  // TODO handle error

        // TODO handle other commands types
        let Command::SimpleCommand{path: cmd_path, args: cmd_args} = self;

        // Child process executing the command
        let mut child = std::process::Command::new(&cmd_path) // TODO handle error
                .args(cmd_args)
                .stdin(Stdio::inherit())
                .spawn()
                .expect("command failed"); // TODO handle error

        child.wait().expect("wait error"); // TODO handle error

    }

}
