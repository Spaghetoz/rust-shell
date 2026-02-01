mod common;

use std::{env, fs};

use rust_shell::command::{Command, RedirectionType};
use crate::common::{TempFiles, create_test_io_context};


// TODO fix this test, it might fail sometimes due to existing file conflicts
/// Tests that the "echo -n hello" command writes the correct output on stdout
/*#[test]
fn test_echo_with_args_simple_command() {
    let mut temp_files = TempFiles::new();
    let io_context = create_test_io_context(&mut temp_files);
    
    let cmd = Command::SimpleCommand { 
        path: "echo".to_string(), 
        args: vec!["-n".to_string(), "hello".to_string()] 
    };
    
    cmd.execute(&io_context).unwrap();
    drop(io_context);
    
    let output = fs::read_to_string(temp_files.stdout_path()).unwrap();
    assert_eq!(output, "hello");
}*/

/// Tests that "echo hello > rust_shell_test.txt" writes on the file
#[test]
fn test_echo_redirection_on_file() {

    let redirected_file = env::temp_dir().join(format!("rust_shell_test_{}.txt", std::process::id()));

    let cmd = Command::Redirection { 
        kind: RedirectionType::Out, 
        command: Box::new(Command::SimpleCommand { 
            path: "echo".to_string(), 
            args: vec!["hello".to_string()] 
        }), 
        file: redirected_file.to_string_lossy().to_string(),
    };

    cmd.execute(&create_test_io_context(&mut TempFiles::new())).unwrap();

    // check that the file was created
    assert!(redirected_file.exists(), "File wasn't created");

    // check that the content is correct
    let content = fs::read_to_string(&redirected_file).unwrap();
    assert_eq!(content.trim(), "hello"); 
    
    fs::remove_file(redirected_file).ok();
}

/// Tests that the ">" redirection overrides the old content of the file if it exists
#[test]
fn test_redirection_overwrite() {
    let mut temp_files = TempFiles::new();
    let file_path = temp_files.add(
        std::env::temp_dir().join(format!("test_overwrite_{}.txt", std::process::id()))
    );

    // Writes some initial content
    std::fs::write(&file_path, "old content").unwrap();

    let cmd = Command::Redirection {
        kind: RedirectionType::Out,
        command: Box::new(Command::SimpleCommand {
            path: "echo".to_string(),
            args: vec!["new content".to_string()],
        }),
        file: file_path.to_string_lossy().to_string(),
    };

    cmd.execute(&create_test_io_context(&mut temp_files)).unwrap();

    let output = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(output.trim(), "new content");
}


// Tests that ">>" on an existing file doesn't overrides the data and correctly appends to the existing content
#[test]
fn test_redirection_append() {
    let mut temp_files = TempFiles::new();
    let file_path = temp_files.add(
        std::env::temp_dir().join(format!("test_append_{}.txt", std::process::id()))
    );

    std::fs::write(&file_path, "first line\n").unwrap();

    let cmd = Command::Redirection {
        kind: RedirectionType::Append, // >>
        command: Box::new(Command::SimpleCommand {
            path: "echo".to_string(),
            args: vec!["second line".to_string()],
        }),
        file: file_path.to_string_lossy().to_string(),
    };

    cmd.execute(&create_test_io_context(&mut temp_files)).unwrap();

    let output = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(output, "first line\nsecond line\n"); // echo adds \n
}

//TODO Tests that cat < somefile.txt correctly writes the file content on iocontext's stdout 
//TODO test ">>" on non existing file
//TODO test chained pipe and redirections