use crate::error::ShellError;
use crate::parse::{Args, ParsedCommand, RunTimeEnvPath, ShellResult};
use crate::shellio::{IOHandler, IOMode};
use std::fs::{self, File};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::{self, Stdio};

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

pub fn execute_external(program: &String, args: Args, io_handler: &IOHandler) -> ShellResult {
    let out = match io_handler.stdout_mode {
        IOMode::INHERIT => Stdio::inherit(),
        IOMode::FILE => {
            let file_handle = File::create(io_handler.stdout_redirect_path.clone())?;
            Stdio::from(file_handle)
        }
        _ => Stdio::null(),
    };
    let err = match io_handler.stderr_mode {
        IOMode::INHERIT => Stdio::inherit(),
        IOMode::FILE => {
            let file_handle = File::create(io_handler.stderr_redirect_path.clone())?;
            Stdio::from(file_handle)
        }
        _ => Stdio::null(),
    };
    match process::Command::new(program)
        .args(args)
        .stdout(out)
        .stderr(err)
        .status()
    {
        Ok(_) => Ok(1),
        _ => Err(ShellError::ExecuteError(program.to_string())),
    }
}
