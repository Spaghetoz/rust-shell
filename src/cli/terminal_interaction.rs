
//! 
//! note: This module encapsulates the third party lib used for
//! enriched user input (navigation in the input with arrow, shortcuts handling (ctrl c, selecting text, copy paste) etc...) 
//! 
//! 
use std::{error::Error};
use rustyline::{DefaultEditor, error::ReadlineError};

use crate::{cli::interaction::{Interaction, UserInput}, command::builtin::get_working_directory};

// TODO more doc
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
        
    // Returns the prefix on the left of the user input
    fn get_prompt_string(&self) -> String {

        let mut prompt_string = String::new();
        
        let working_dir = get_working_directory()
            .unwrap_or_else(|_| String::from("unknown"));

        // pretty colored shell prompt
        prompt_string.push_str(&format!("$ \x1b[1;34m{}\x1b[0m> ", working_dir));
        
        prompt_string
    }

}

impl Interaction for TerminalInteraction {

    /// Returns the String entered by the user on the stdin
    fn receive_input(&mut self) -> Result<UserInput, Box<dyn Error>> {

        // side effect: also prints the prompt string
        let readline = self.rusty_lines_editor.readline(&self.get_prompt_string());
        match readline {
            Ok(line) => {
                //rl.add_history_entry(line.as_str())?;
                Ok(UserInput::String(line))
            },
            Err(ReadlineError::Interrupted) => {
                Ok(UserInput::Interruption)
            },
            Err(_) => Ok(UserInput::String("".to_string()))  // TODO handle other cases
        }

    }

    fn save_history(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
