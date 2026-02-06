
//!
//! Module related with the interactions between with the user. 
//! 

use std::{error::Error};

/// Represents the contract that an interaction with the user should respect
pub trait Interaction {
    fn receive_input(&mut self) -> Result<UserInput, Box<dyn Error>>;
    fn save_history(&mut self) -> Result<(), Box<dyn Error>>;
}

/// Represents what a user input could be, it could be just a string, or an action 
pub enum UserInput {
    String(String),
    Interruption, // ctrl c
    Eof           // ctrl d
}

// TODO custom errors
/*pub enum InteractionError {
    InputError;
}*/