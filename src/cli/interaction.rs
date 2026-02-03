
//!
//! Module related with the interactions between the user and the terminal. 
//! 

use std::{error::Error, io::{self, Write}};

use rustyline::{DefaultEditor, error::ReadlineError};

/// Represents the contract that an interaction with the user should respect
pub trait Interaction {
    fn receive_input(&mut self) -> Result<UserInput, Box<dyn Error>>;
    fn save_history(&mut self) -> Result<(), Box<dyn Error>>;
}
/// Represents what a user input could be, it could be just a string, or an action 
pub enum UserInput {
    String(String),
    Interruption, // ctrl c
}


pub struct TerminalInteraction {

    rusty_lines_editor: DefaultEditor
}

impl TerminalInteraction {
    
    pub fn try_new() -> Result<Self, Box<dyn std::error::Error>> {   

        let rusty_lines_editor = DefaultEditor::new()?;

        Ok(TerminalInteraction {
            rusty_lines_editor
        })
    }
}

impl Interaction for TerminalInteraction {

    /// Returns the String entered by the user on the stdin
    fn receive_input(&mut self) -> Result<UserInput, Box<dyn Error>> {

        let readline = self.rusty_lines_editor.readline("");
        match readline {
            Ok(line) => {
                //rl.add_history_entry(line.as_str())?;
                Ok(UserInput::String(line))
            },
            Err(ReadlineError::Interrupted) => {
                Ok(UserInput::Interruption)
            },
            Err(err) => Ok(UserInput::String("".to_string()))  // TODO handle other cases
        }

    }


    fn save_history(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}


/*pub enum InteractionError {
    InputError;
}*/