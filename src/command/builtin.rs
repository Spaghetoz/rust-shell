
//! Shell commands that needs a special treatment, 
//! for instance exit and cd
//!  

pub fn exit_shell(exit_code: i32) {
    std::process::exit(exit_code)
}

pub fn change_directory(to: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let path = std::path::Path::new(to);
    std::env::set_current_dir(&path)?;

    Ok(())
}