use crate::parse::{self, ParsedCommand};
use std::path::PathBuf;

const BUILT_IN_COMMANDS: [&str; 3] = ["exit", "type", "echo"];

pub fn not_found(parsed_command: ParsedCommand) {
    println!("{}: command not found", parsed_command.command);
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

fn search_file_in_paths(filename: &str, paths: &[&str]) -> Option<PathBuf> {
    paths.iter().find_map(|dir| {
        let full_path = PathBuf::from(dir).join(filename);
        if full_path.is_file() {
            Some(full_path)
        } else {
            None
        }
    })
}
