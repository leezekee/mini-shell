#[allow(unused_imports)]
use std::io::{self, Write};

use crate::{
    parse::{CommandHandler, ParsedCommand},
    shellio::IOHandler,
};
pub mod command;
pub mod error;
pub mod parse;
pub mod shellio;
pub mod utils;

fn main() {
    let mut io_handler = IOHandler::new();
    let mut command_handler = CommandHandler::new();
    loop {
        IOHandler::print_prompt();
        exec_command(&mut command_handler, &mut io_handler);
        io_handler.reset();
    }
}

fn exec_command(command_handler: &mut CommandHandler, io_handler: &mut IOHandler) {
    let mut raw_command: String;
    match IOHandler::get_raw_command() {
        Ok(mut r_cmd) => raw_command = std::mem::take(&mut r_cmd),
        Err(_) => {
            return;
        }
    }

    let parsed_command = parse::parse(&mut raw_command);
    let command: ParsedCommand;
    match parsed_command {
        Ok(cmd) => command = cmd,
        Err(e) => {
            io_handler.stderr(format_args!("{}", e));
            return;
        }
    }
    IOHandler::debug(format_args!("{:?}", command));
    if !command.stdout.is_empty() {
        io_handler.stdout_mode = shellio::IOMode::FILE;
        io_handler.stdout_redirect_path = command.stdout.clone();
    }

    if !command.stderr.is_empty() {
        io_handler.stderr_mode = shellio::IOMode::FILE;
        io_handler.stderr_redirect_path = command.stderr.clone();
    }

    match command_handler.run(command, io_handler) {
        Ok(_) => {}
        Err(e) => io_handler.stderr(format_args!("{}", e)),
    }
}
