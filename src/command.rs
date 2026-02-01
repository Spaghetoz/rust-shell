
//! Module related with commands execution, treatment etc
//! 
//! 

pub mod execution;
pub mod builtin;

use std::sync::Arc;
use std::fs::File;

pub enum Command {
    SimpleCommand {
        path: String,
        args: Vec<String>,
    },
    /*Pipe {
        left: Box<Command>,
        right: Box<Command>,
    },*/
    Redirection {
        kind: RedirectionType,
        command: Box<Command>,
        file: String,
    },
}

pub enum RedirectionType {
    //In,       // <
    Out,      // >
    //Append,   // >>
    //Err,      // 2>
}

/// Struct containing what stdin should be and where stdout and stderr should go
/// It may be used to specify redirections and pipe destinations, and be used for testing
#[derive(Clone)]
pub struct IoContext {
    // Use of Arc to make File cloneable
    pub stdin: Arc<File>, 
    pub stdout: Arc<File>,
    pub stderr: Arc<File>,
}