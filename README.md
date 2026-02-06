# Rust (mini)-shell
![rust](https://img.shields.io/badge/Rust-444444?logo=rust&logoColor=red)
![Last Commit](https://img.shields.io/github/last-commit/spaghetoz/rust-shell)

### A basic unix shell made with Rust

### Supported platforms: 
- Linux (tested on Ubuntu WSL)
- Maybe MacOS

### Features
- Commands parsing
- Commands execution : 
    - Simple commands (for example `ls -l /`)
    - Basic redirections (<, >>, >>, 2>)
    - Pipes commands
- Commands chaining (with ;)
- Pipes chaining and partially redirections chaining
- Enriched line editing and history thanks to the [Rusty lines](https://github.com/kkawakam/rustyline) library

### How to use : `cargo run` 
