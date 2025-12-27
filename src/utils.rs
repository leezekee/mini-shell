use crate::error::ShellError;
use crate::parse::{Args, ParsedCommand, RunTimeEnvPath, ShellResult};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process;

pub fn search_file_in_paths(filename: &String, paths: RunTimeEnvPath) -> Option<PathBuf> {
    paths.borrow().iter().find_map(|dir| {
        let full_path = PathBuf::from(dir).join(filename);
        if full_path.is_file() && is_executable(&full_path) {
            Some(full_path)
        } else {
            None
        }
    })
}

pub fn is_executable(file_path: &PathBuf) -> bool {
    match fs::metadata(file_path) {
        Ok(metadata) => {
            let mode = metadata.permissions().mode();
            mode & 0o111 != 0
        }
        Err(_) => false,
    }
}

pub fn not_found(parsed_command: ParsedCommand) {
    println!("{}: command not found", parsed_command.command);
}

pub fn execute_external(program: &String, args: Args) -> ShellResult {
    let status = process::Command::new(program)
        .args(&args)
        .status()
        .map_err(|e| ShellError::ProcessStartError {
            cmd: program.to_string(),
            source: e,
        })?;
    if !status.success() {
        return Err(ShellError::ProcessExitError {
            cmd: program.to_string(),
            code: status.code().unwrap_or(-1),
        });
    }

    Ok(1)
}
