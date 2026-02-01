mod common;

use std::fs;

use rust_shell::command::Command;
use crate::common::{TempFiles, create_test_io_context};


/// Tests the echo -n hello command execution
#[test]
fn test_echo_with_args_simple_command() {
    
    let mut temp_files = TempFiles::new();
    let io_context = create_test_io_context(&mut temp_files);

    // This command should output on stdout : "hello" without \n at the end
    let cmd = Command::SimpleCommand { 
        path: "echo".to_string(), 
        args: vec!["-n".to_string(), "hello".to_string()] 
    };
    
    cmd.execute(&io_context).unwrap();

    drop(io_context);

    let stdout_path = &temp_files.paths[1]; // stdout is the 2nd
    let output = fs::read_to_string(stdout_path).unwrap();
    assert_eq!(output, "hello");
}

