//!
//! Utils for testing commands execution
//! 

use rust_shell::command::{IoContext};

use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::fs::{OpenOptions};

// Stores the paths of the temporary test files
pub struct TempFiles {
    pub paths: Vec<PathBuf>,
}

impl TempFiles {
    pub fn new() -> Self {
        TempFiles { paths: Vec::new() }
    }

    pub fn add(&mut self, path: PathBuf) -> PathBuf {
        self.paths.push(path.clone());
        path
    }
}

// Automatic test files clean 
impl Drop for TempFiles {
    fn drop(&mut self) {
        for path in &self.paths {
            fs::remove_file(path).ok();
        }
    }
}

/// Creates files that acts as stdout, stdin, stderr
pub fn create_test_io_context(temp_files: &mut TempFiles) -> IoContext {
    let pid = std::process::id();
    
    let stdin_path = temp_files.add(
        std::env::temp_dir().join(format!("test_stdin_{}.txt", pid))
    );
    let stdout_path = temp_files.add(
        std::env::temp_dir().join(format!("test_stdout_{}.txt", pid))
    );
    let stderr_path = temp_files.add(
        std::env::temp_dir().join(format!("test_stderr_{}.txt", pid))
    );

    let stdin = Arc::new(OpenOptions::new().read(true).write(true).create(true).open(&stdin_path).unwrap());
    let stdout = Arc::new(OpenOptions::new().write(true).create(true).truncate(true).open(&stdout_path).unwrap());
    let stderr = Arc::new(OpenOptions::new().write(true).create(true).truncate(true).open(&stderr_path).unwrap());
    IoContext { stdin, stdout, stderr }
}