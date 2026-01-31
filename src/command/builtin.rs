
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

pub fn get_working_directory() -> Result<String, Box<dyn std::error::Error>> {
    
    let path = std::env::current_dir()?;
    Ok(path.to_string_lossy().into_owned())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cd_root_sets_working_directory_to_root() {
        change_directory("/").unwrap();
        let working_dir = get_working_directory().unwrap();
        assert_eq!("/", working_dir);
    }

    #[test]
    fn cd_home_sets_working_directory_to_home() {
        let home = std::env::home_dir().unwrap();
        change_directory(home.to_str().unwrap()).unwrap();
        let working_dir = get_working_directory().unwrap();
        assert_eq!(home.to_str().unwrap(), working_dir);
    }
}