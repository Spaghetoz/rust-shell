
use crate::command::{Command};

#[derive(Clone)]
pub enum Token {
    Word(String),
    RedirectOp(String),
    Pipe
}

/// Converts a string representing a command into a Command structure
/// For example "ls /home" gives SimpleCommand("ls", ["/home"])
pub fn convert_to_command(input: &str) -> Result<Command, Box<dyn std::error::Error>>  {
    
    // Turns the input in a vec of Strings 
    let input_tokens = tokenize_input(&input);
    // Turns the tokens into a command structure
    let command = parse(&input_tokens)?;

    Ok(command)
}

/// Converts an input string into a vec of tokens
fn tokenize_input(input: &str) -> Vec<Token> { 

    let mut tokens: Vec<Token> = Vec::new();

    for word in input.split_whitespace() {
        tokens.push(match word {   
            "<" | ">" | ">>" | "2>" => Token::RedirectOp(word.to_string()), // TODO token type for each variant instead of putting string in RedirectOp
            "|" => Token::Pipe,
            _ => Token::Word(word.to_string())
        }); 
    }

    tokens
}

fn parse(tokens: &[Token]) -> Result<Command, Box<dyn std::error::Error>> {

    let mut tokens_group: Vec<Token> = Vec::new();

    for token in tokens.iter() {

        match token {
            // If no special operator, simply put the word in the tokens group, and treat them later
            Token::Word(_) => tokens_group.push(token.clone()), // TODO avoid clone
            Token::RedirectOp(_) => todo!(),
            Token::Pipe => todo!(),
        }   
    }

    Ok(create_simple_command(&tokens_group)?)
}

// Creates (if the tokens are well formed) a simple command
fn create_simple_command(tokens: &[Token]) -> Result<Command, Box<dyn std::error::Error>> {

    let Token::Word(cmd_path) = tokens.get(0).ok_or("missing first token")? else {
        return Err("expected a word token".into());
    };

    let cmd_args: Vec<String> = tokens[1..].iter().map(|token| 
        match token {
            Token::Word(arg) => Ok(arg.clone()), // TODO avoid clone
            _ => Err("token should be a word"),
        })
        .collect::<Result<_, _>>()?;

    Ok(Command::Simple { cmd_path: cmd_path.clone(), cmd_args: cmd_args })
}