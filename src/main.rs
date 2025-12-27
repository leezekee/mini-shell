#[allow(unused_imports)]
use std::io::{self, Write};

use crate::parse::{CommandHandler, ParsedCommand};
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
        exec_command(&mut command_handler, &mut raw_command);
    }
}

fn exec_command(command_handler: &mut CommandHandler, raw_command: &mut String) {
    let parsed_command = parse::parse(raw_command);
    let command: ParsedCommand;
    match parsed_command {
        Ok(cmd) => command = cmd,
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    println!("Command: {}", command.command);
    println!("Args: {:?}", command.args);
    match command_handler.run(command) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}
