use std::io;
use thiserror::Error;

use crate::parse::BuiltIn;

#[derive(Debug, Error)]
pub enum ShellError {
    #[error("Not a builtin command")]
    NotABuiltinCommand,
    #[error("{0}: not found")]
    CommandNotFound(String),
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Failed to execute command '{cmd}': {source}")]
    ProcessStartError {
        cmd: String,
        #[source]
        source: io::Error,
    },

    #[error("{}: execute failed", {0})]
    ExecuteError(String),

    #[error("Command '{cmd}' failed with exit code {code}")]
    ProcessExitError { cmd: String, code: i32 },

    #[error("{cmd}: {dir}: No such file or directory")]
    DirectoryNotExist { cmd: BuiltIn, dir: String },

    #[error("Invalid syntax!")]
    InvalidSyntax,

    #[error("")]
    NullInput,
}
