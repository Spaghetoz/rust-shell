
//! Module related with commands execution, treatment etc
//! 
//! 

use std::{fs::File, process::Stdio};

pub mod execution;
pub mod builtin;

#[derive(PartialEq, Debug)]
pub enum Command {
    Simple {
        cmd_path: String,
        cmd_args: Vec<String>,
    },
    Pipe {
        left: Box<Command>,
        right: Box<Command>,
    },
    Redirection {
        kind: RedirectionType,
        command: Box<Command>,
        file: String,
    },
}

#[derive(Clone, PartialEq, Debug)]
pub enum RedirectionType {
    In,       // <
    Out,      // >
    Append,   // >>
    Err,      // 2>
}

// TODO update comments
/// Struct containing what stdin should be and where stdout and stderr should go.
/// It may be used to specify redirections and pipe destinations, and be used for testing
pub struct IoContext {
    pub stdin: Stdio, 
    pub stdout: Stdio,
    pub stderr: Stdio,
}

impl IoContext {

    pub fn new() -> Self {
        IoContext { 
            // By default use the parent process stdout stderr and stdout
            stdin: Stdio::inherit(), 
            stdout: Stdio::inherit(), 
            stderr: Stdio::inherit() 
        }
    }
}