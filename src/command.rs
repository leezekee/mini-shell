#![allow(unused_variables)]
use crate::error::ShellError;
use crate::parse::{self, Arg, BuiltIn, ParsedCommand, RunTimeEnvPath, ShellResult};
use crate::shellio::IOHandler;
use crate::utils::*;
use std::env;
use std::path::Path;
pub fn not_found(parsed_command: ParsedCommand, paths: RunTimeEnvPath, io_handler: &IOHandler) {
    io_handler.stdout(format_args!(
        "{}: command not found",
        parsed_command.command
    ));
}

pub fn echo(
    parsed_command: ParsedCommand,
    paths: RunTimeEnvPath,
    io_handler: &IOHandler,
) -> ShellResult {
    io_handler.stdout(format_args!("{}", parsed_command.args.join(" ")));
    return Ok(1);
}

pub fn _exit(
    parsed_command: ParsedCommand,
    paths: RunTimeEnvPath,
    io_handler: &IOHandler,
) -> ShellResult {
    std::process::exit(0)
}

pub fn _type(
    parsed_command: ParsedCommand,
    paths: RunTimeEnvPath,
    io_handler: &IOHandler,
) -> ShellResult {
    let command = parsed_command.args.join(" ");
    match command.parse::<BuiltIn>() {
        Ok(cmd) => {
            io_handler.stdout(format_args!("{} is a shell builtin", cmd));
            Ok(1)
        }
        _ => match search_file_in_paths(&command, paths) {
            Some(path) => {
                io_handler.stdout(format_args!("{} is {}", command, path.display()));
                Ok(1)
            }
            None => Err(ShellError::CommandNotFound(command)),
        },
    }
}

pub fn pwd(
    parsed_command: ParsedCommand,
    paths: RunTimeEnvPath,
    io_handler: &IOHandler,
) -> ShellResult {
    let work_dir = env::current_dir().expect("");
    io_handler.stdout(format_args!("{}", work_dir.display()));
    return Ok(1);
}

pub fn cd(
    parsed_command: ParsedCommand,
    paths: RunTimeEnvPath,
    io_handler: &IOHandler,
) -> ShellResult {
    let mut target_dir: &Arg = &parsed_command.args[0];
    let home = parse::get_env_home();
    if target_dir == "~" {
        target_dir = &home;
    }
    let path = Path::new(&target_dir);
    match env::set_current_dir(path) {
        Ok(_) => Ok(1),
        Err(_) => Err(ShellError::DirectoryNotExist {
            cmd: BuiltIn::CD,
            dir: target_dir.to_string(),
        }),
    }
}
