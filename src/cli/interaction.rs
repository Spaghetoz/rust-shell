
//!
//! Module related with the interactions between the user and the terminal. 
//! 

use std::{error::Error, io};

/// Represents the contract that an interaction with the user should respect
pub trait Interaction {
    fn receive_input(&self) -> Result<UserInput, Box<dyn Error>>;
    fn save_history(&self) -> Result<(), Box<dyn Error>>;
}
/// Represents what a user input could be, it could be just a string, or an action 
pub enum UserInput {
    String(String),
    Interruption, // ctrl c
}


pub struct TerminalInteraction {

}

impl TerminalInteraction {
    
    pub fn new() -> Self {
        TerminalInteraction {}
    }
}

impl Interaction for TerminalInteraction {

    /// Returns the String entered by the user on the stdin
    fn receive_input(&self) -> Result<UserInput, Box<dyn Error>> {

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        Ok(UserInput::String(input))
        
    }


    fn save_history(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}


/*pub enum InteractionError {
    InputError;
}*/