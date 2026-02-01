
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
/// It may be used to specify redirections and pipe destinations, and be used for testing
pub struct IoContext {
    pub stdin: std::fs::File, 
    pub stdout: std::fs::File,
    pub stderr: std::fs::File,
}