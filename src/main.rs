#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        match command.trim() {
            "exit" => break,
            _ => command_not_found(&mut command),
        }
    }
}

fn command_not_found(command: &mut String) {
    println!("{}: command not found", command.trim());
}
