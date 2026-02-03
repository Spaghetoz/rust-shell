
//!
//! Module related with the interactions between the user and the terminal. 
//! 

use std::{error::Error, io};

/// Represents the contract that an interaction with the user should respect
pub trait Interaction {
    fn receive_input(&self) -> Result<String, Box<dyn Error>>;
    fn save_history(&self) -> Result<(), Box<dyn Error>>;
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
    fn receive_input(&self) -> Result<String, Box<dyn Error>> {

        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_string();

        Ok(input)
        
    }


    fn save_history(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}


/*pub enum InteractionError {
    InputError;
}*/