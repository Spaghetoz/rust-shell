
use crate::command::{Command};

pub enum Token {
    Word(String),
    RedirectOp(String),
    Pipe
}

/// Converts a string representing a command into a Command structure
/// For example "ls /home" gives SimpleCommand("ls", ["/home"])
pub fn convert_to_command(input: &str) -> Command {
    
    // Turns the input in a vec of Strings 
    let input_tokens = tokenize_input(&input);
    // Turns the tokens into a command structure
    let command = parse(&input_tokens);

    command
}

/// Converts an input string into a vec of tokens
fn tokenize_input(input: &str) -> Vec<Token> { 

    let mut tokens: Vec<Token> = Vec::new();

    for word in input.split_whitespace() {
        tokens.push(match word {   
            "<" | ">" | ">>" | "2>" => Token::RedirectOp(word.to_string()),
            "|" => Token::Pipe,
            _ => Token::Word(word.to_string())
        }); 
    }

    tokens
}

fn parse(tokens: &[Token]) -> Command {

    // TODO handle other commands types than simple commands
    let Token::Word(command_path) = tokens.get(0).expect("arg 0 not found") else {
        panic!("unsupported token"); 
    };

    let mut args : Vec<String> = Vec::new();
    for token in &tokens[1..] {
        match token {
            Token::Word(arg) => args.push(arg.clone()),
            _ => panic!("unsupported yet"),
        }
    }

    Command::Simple { cmd_path: command_path.clone(), cmd_args: args}
}