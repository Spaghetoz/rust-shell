

/// Converts an input string into a vec of strings 
/// For example "ls -l ." becomes a vec of ["ls", "-l", "."]
pub fn tokenize_input(input: &str) -> Vec<&str> {   // TODO return a custom command struct

    let mut tokens: Vec<&str> = Vec::new();

    for token in input.split_whitespace() {
        tokens.push(token);
    }

    tokens
}