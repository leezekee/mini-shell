#[allow(unused_imports)]
use std::io::{self, Write};

use crate::parse::CommandHandler;
pub mod command;
pub mod error;
pub mod parse;
pub mod utils;

fn main() {
    let mut command_handler = CommandHandler::new();
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
        command_handler.run(command);
    }
}
