use std::io;


pub fn run_cli() {

    println!("--- Rust shell ---");

    loop {
        let user_input = receive_input();
        println!("{user_input}");
    }
}

fn receive_input() -> String {

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_string();

    input
    
}
