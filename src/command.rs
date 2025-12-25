use crate::parse::{self, ParsedCommand};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process;
use std::{env, fs};

const BUILT_IN_COMMANDS: [&str; 4] = ["exit", "type", "echo", "pwd"];

pub fn default(parsed_command: ParsedCommand) {
    let paths = parse::get_env_path();
    match search_file_in_paths(parsed_command.command, &paths) {
        Some(_) => execute_external(parsed_command.command, parsed_command.args),
        None => not_found(parsed_command),
    }
}

pub fn echo(parsed_command: ParsedCommand) {
    println!("{}", parsed_command.args.join(" "));
}

pub fn _type(parsed_command: ParsedCommand) {
    let mut command = parsed_command.args.join(" ");
    let paths = parse::get_env_path();
    if BUILT_IN_COMMANDS.contains(&command.as_ref()) {
        println!("{} is a shell builtin", command)
    } else {
        // search arguments in paths
        match search_file_in_paths(command.as_mut_str(), &paths) {
            Some(path) => println!("{} is {}", command, path.display()),
            None => println!("{}: not found", command),
        }
    }
}

pub fn pwd() {
    let work_dir = env::current_dir().expect("");
    println!("{}", work_dir.display());
}

// ================== private functions ==================

fn search_file_in_paths(filename: &str, paths: &[&str]) -> Option<PathBuf> {
    paths.iter().find_map(|dir| {
        let full_path = PathBuf::from(dir).join(filename);
        if full_path.is_file() && is_executable(&full_path) {
            Some(full_path)
        } else {
            None
        }
    })
}

fn is_executable(file_path: &PathBuf) -> bool {
    match fs::metadata(file_path) {
        Ok(metadata) => {
            let mode = metadata.permissions().mode();
            mode & 0o111 != 0
        }
        Err(_) => false,
    }
}

fn not_found(parsed_command: ParsedCommand) {
    println!("{}: command not found", parsed_command.command);
}

#[allow(unused_variables)]
fn execute_external(program: &str, args: parse::Args) {
    let status = process::Command::new(program)
        .args(&args)
        .status()
        .expect("");
}
