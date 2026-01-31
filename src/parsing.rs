

/// Converts an input string into a vec of strings 
/// For example "ls -l ." becomes a vec of ["ls", "-l", "."]
pub fn tokenize_input(input: String) -> Vec<String> {   // TODO return a custom command struct

    let mut tokens: Vec<String> = Vec::new();

    for token in input.split_whitespace() {
        tokens.push(token.to_string());
    }

    tokens
}