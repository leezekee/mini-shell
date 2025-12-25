#[allow(unused_imports)]
use std::io::{self, Write};
pub mod command;
pub mod parse;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut raw_command = String::new();
        io::stdin().read_line(&mut raw_command).unwrap();
        let parsed_command = parse::parse(&mut raw_command);
        if let None = parsed_command {
            continue;
        }
        let command = parsed_command.unwrap();
        match command.as_ref() {
            "exit" => break,
            "echo" => command::echo(command),
            "type" => command::_type(command),
            _ => command::not_found(command),
        }
    }
}
