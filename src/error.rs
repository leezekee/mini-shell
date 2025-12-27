use std::io;
use thiserror::Error;

use crate::parse::BuiltIn;

#[derive(Debug, Error)]
pub enum ShellError {
    #[error("Not a builtin command")]
    NotABuiltinCommand,
    #[error("{}: not found", {0})]
    CommandNotFound(String),
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Failed to execute command '{cmd}': {source}")]
    ProcessStartError {
        cmd: String,
        #[source]
        source: io::Error,
    },

    #[error("Command '{cmd}' failed with exit code {code}")]
    ProcessExitError { cmd: String, code: i32 },

    #[error("{cmd}: {dir}: No such file or directory")]
    DirectoryNotExist { cmd: BuiltIn, dir: String },
}
