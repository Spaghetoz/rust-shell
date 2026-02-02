
//! Module related with commands execution, treatment etc
//! 
//! 

pub mod execution;
pub mod builtin;

pub enum Command {
    Simple(SimpleCommand),
    /*Pipe {
        left: Box<Command>,
        right: Box<SimpleCommand>,
    },*/
    Redirection {
        kind: RedirectionType,
        command: Box<Command>,
        file: String,
    },
}

pub struct SimpleCommand {
    pub path: String,
    pub args: Vec<String>,
}


pub enum RedirectionType {
    In,       // <
    Out,      // >
    Append,   // >>
    Err,      // 2>
}

// TODO update comments
/// Struct containing what stdin should be and where stdout and stderr should go.
/// It may be used to specify redirections and pipe destinations, and be used for testing
#[derive(Clone)]
pub struct IoFds {
    pub stdin: i32, 
    pub stdout: i32,
    pub stderr: i32,
}