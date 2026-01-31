
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
