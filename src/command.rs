#![allow(unused_variables)]
use crate::parse::{self, Arg, BuiltIn, ParsedCommand, RunTimeEnvPath};
use crate::utils::*;
use std::env;
use std::path::Path;

pub fn not_found(parsed_command: ParsedCommand, paths: RunTimeEnvPath) {
    println!("{}: command not found", parsed_command.command);
}

pub fn echo(parsed_command: ParsedCommand, paths: RunTimeEnvPath) {
    println!("{}", parsed_command.args.join(" "));
}

pub fn _exit(parsed_command: ParsedCommand, paths: RunTimeEnvPath) {
    std::process::exit(0)
}

pub fn _type(parsed_command: ParsedCommand, paths: RunTimeEnvPath) {
    let command = parsed_command.args.join(" ");
    match command.parse::<BuiltIn>() {
        Ok(cmd) => println!("{} is a shell builtin", cmd),
        _ => match search_file_in_paths(&command, paths) {
            Some(path) => println!("{} is {}", command, path.display()),
            None => println!("{}: not found", command),
        },
    }
}

pub fn pwd(parsed_command: ParsedCommand, paths: RunTimeEnvPath) {
    let work_dir = env::current_dir().expect("");
    println!("{}", work_dir.display());
}

pub fn cd(parsed_command: ParsedCommand, paths: RunTimeEnvPath) {
    let mut target_dir: &Arg = &parsed_command.args[0];
    let home = parse::get_env_home();
    if target_dir == "~" {
        target_dir = &home;
    }
    let path = Path::new(&target_dir);
    match env::set_current_dir(path) {
        Ok(_) => {}
        Err(_) => println!("cd: {}: No such file or directory", target_dir),
    }
}
