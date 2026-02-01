
//! Module related with commands execution, treatment etc
//! 
//! 

pub mod execution;
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

/// Struct containing what stdin should be and where stdout and stderr should go
/// It may be used to specify redirections and pipe destinations, and be used for testing by
/// putting the stdout in a String buffer for instance 
pub struct IoContext {
    pub stdin: Box<dyn std::io::Read>,  // TODO use lifetimes
    pub stdout: Box<dyn std::io::Write>,
    pub stderr: Box<dyn std::io::Write>,
}