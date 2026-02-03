
use crate::command::{Command};

// Use the RedirectionType enum for both the tokens (in the lexing) and the AST (in the Command enum)
use crate::command::RedirectionType;

#[derive(Clone)]
pub enum Token {
    Word(String),
    RedirectOp(RedirectionType),
    Pipe
}

/// Converts a string representing a command into a Command structure
/// For example "ls /home" gives SimpleCommand("ls", ["/home"])
pub fn convert_to_command(input: &str) -> Result<Command, Box<dyn std::error::Error>>  {
    
    // Turns the input in a vec of Strings 
    let input_tokens = tokenize_input(input);
    // Turns the tokens into a command structure
    let command = parse(&input_tokens)?;

    Ok(command)
}

/// Converts an input string into a vec of tokens
fn tokenize_input(input: &str) -> Vec<Token> { 

    let mut tokens: Vec<Token> = Vec::new();

    for word in input.split_whitespace() {
        tokens.push(match word {   
            "<" => Token::RedirectOp(RedirectionType::In), 
            ">" => Token::RedirectOp(RedirectionType::Out),
            ">>" => Token::RedirectOp(RedirectionType::Append),
            "2>" => Token::RedirectOp(RedirectionType::Err),
            "|" => Token::Pipe,
            _ => Token::Word(word.to_string())
        }); 
    }

    tokens
}

fn parse(tokens: &[Token]) -> Result<Command, Box<dyn std::error::Error>> {

    let mut visited_tokens: Vec<Token> = Vec::new();

    for (i, token) in tokens.iter().enumerate() {

        match token {
            // If no special operator, simply put the word in the tokens group, and treat them later
            Token::Word(_) => visited_tokens.push(token.clone()), // TODO avoid clone

            Token::RedirectOp(operator) => return create_redirection_command(operator,&visited_tokens, &tokens[i+1..]),
            Token::Pipe => return create_pipe_command(&visited_tokens, &tokens[i+1..]),
        }   
    }

    // If there is no more tokens to process, it's a simple command
    create_simple_command(&visited_tokens)
}

// Creates (if the tokens are well formed) a simple command
fn create_simple_command(tokens: &[Token]) -> Result<Command, Box<dyn std::error::Error>> {

    let Token::Word(cmd_path) = tokens.first().ok_or("missing first token")? else {
        return Err("expected a word token".into());
    };

    let cmd_args: Vec<String> = tokens[1..].iter().map(|token| 
        match token {
            Token::Word(arg) => Ok(arg.clone()), // TODO avoid clone
            _ => Err("token should be a word"),
        })
        .collect::<Result<_, _>>()?;

    Ok(Command::Simple { cmd_path: cmd_path.clone(), cmd_args })
}

fn create_pipe_command(left_tokens: &[Token], right_tokens: &[Token]) -> Result<Command, Box<dyn std::error::Error>> {

    Ok(Command::Pipe {
        // Recursively parse the left and right tokens
        left: Box::new(parse(left_tokens)?),  
        right: Box::new(parse(right_tokens)?),
    })

}

fn create_redirection_command(op: &RedirectionType, left_tokens: &[Token], right_tokens: &[Token]) -> Result<Command, Box<dyn std::error::Error>> {

    let Token::Word(file_path) = right_tokens.first().ok_or("missing word on the right of redirection")? else {
        return Err("token on the right of redirection OP should be a Word".into());
    };

    Ok(Command::Redirection { 
        kind: op.clone(), // TODO avoid clone 
        // TODO handle commands on the right of the redirection, for example ls > out.txt | wc. because now we simply ignore the right_tokens
        command: Box::new(parse(left_tokens)?), 
        file: file_path.to_string()
    })

}





#[cfg(test)]
mod tests {
    use super::*;

    // Tests that a string input returns the correct Command structure form
    #[test]
    fn test_simple_command_with_args() {

        let input = "ls -lia /".to_string();
        let result = convert_to_command(&input).unwrap();

        let expected = Command::Simple { 
            cmd_path: "ls".to_string(), 
            cmd_args: vec!["-lia".to_string(), "/".to_string()] 
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_simple_pipe_command() {

        let input = "echo hello | cat".to_string();
        let result = convert_to_command(&input).unwrap();

        let expected = Command::Pipe { 
            left: Box::new(Command::Simple{cmd_path: "echo".to_string(), cmd_args: vec!["hello".to_string()]}), 
            right: Box::new(Command::Simple{cmd_path: "cat".to_string(), cmd_args: vec![]}) 
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_out_redirection_command() {
        
        let input = "echo hello > test.txt".to_string();
        let result = convert_to_command(&input).unwrap();

        let expected = Command::Redirection { 
            kind: RedirectionType::Out, 
            command: Box::new(Command::Simple { 
                cmd_path: "echo".to_string(), 
                cmd_args: vec!["hello".to_string()] 
            }), 
            file: "test.txt".to_string()
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_in_redirection_command() {
        
        let input = "cat < input.txt".to_string();
        let result = convert_to_command(&input).unwrap();

        let expected = Command::Redirection { 
            kind: RedirectionType::In, 
            command: Box::new(Command::Simple { 
                cmd_path: "cat".to_string(), 
                cmd_args: vec![] 
            }), 
            file: "input.txt".to_string()
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_append_redirection_command() {
        
        let input = "echo hello >> test.txt".to_string();
        let result = convert_to_command(&input).unwrap();

        let expected = Command::Redirection { 
            kind: RedirectionType::Append, 
            command: Box::new(Command::Simple { 
                cmd_path: "echo".to_string(), 
                cmd_args: vec!["hello".to_string()] 
            }), 
            file: "test.txt".to_string()
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_err_redirection_command() {
        
        let input = "echo hello 2> test.txt".to_string();
        let result = convert_to_command(&input).unwrap();

        let expected = Command::Redirection { 
            kind: RedirectionType::Err, 
            command: Box::new(Command::Simple { 
                cmd_path: "echo".to_string(), 
                cmd_args: vec!["hello".to_string()] 
            }), 
            file: "test.txt".to_string()
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_chained_pipes() {

        let input = "ls -l / | cat | head".to_string();
        let result = convert_to_command(&input).unwrap();

        let expected = Command::Pipe { 
            left: Box::new(Command::Simple { 
                cmd_path: "ls".to_string(), 
                cmd_args: vec!["-l".to_string(), "/".to_string()]
            }), 

            // in the current parsing system, the sub pipes are in the right
            right: Box::new(Command::Pipe {    
                left: Box::new(Command::Simple { 
                    cmd_path: "cat".to_string(), 
                    cmd_args: vec![]
                }),         
                right: Box::new(Command::Simple { 
                    cmd_path: "head".to_string(), 
                    cmd_args: vec![]
                }),
            }), 
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_redirection_after_pipe() {

        let input = "ls -l / | cat >> test.txt".to_string();
        let result = convert_to_command(&input).unwrap();

        let expected = Command::Pipe { 
            left: Box::new(Command::Simple { 
                cmd_path: "ls".to_string(), 
                cmd_args: vec!["-l".to_string(), "/".to_string()]
            }), 

            right: Box::new(Command::Redirection {
                 kind: RedirectionType::Append, 
                 command: Box::new(Command::Simple { 
                    cmd_path: "cat".to_string(), 
                    cmd_args: vec![]
                }), 
                 file: "test.txt".to_string()
            })
        };

        assert_eq!(expected, result);
    }


    // TODO when implemented, test redirection before a pipe : cat < input.txt | head
    // TODO when implemented, test sticked pipe or redirection : echo hello|cat or echo hello>test.txt

}