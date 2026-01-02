use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::process::Stdio;

#[derive(Debug)]
pub enum IOMode {
    PIPED,
    FILE,
    INHERIT,
    NULL,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OutLevel {
    DEBUG = 0,
    INFO,
    WARN,
    ERROR,
}

pub type PipeHandler = Option<Stdio>;

#[derive(Debug)]
pub struct IOHandler {
    pub stdin_mode: IOMode,
    pub stdout_mode: IOMode,
    pub stderr_mode: IOMode,

    pub stdin_redirect_path: String,
    pub stdout_redirect_path: String,
    pub stderr_redirect_path: String,

    pub stdin_pipe: PipeHandler,
    pub stdout_pipe: PipeHandler,
    pub stderr_pipe: PipeHandler,
}

impl IOHandler {
    pub const OUT_LEVEL: OutLevel = OutLevel::INFO;

    pub fn new() -> IOHandler {
        IOHandler {
            stdin_mode: IOMode::INHERIT,
            stdout_mode: IOMode::INHERIT,
            stderr_mode: IOMode::INHERIT,
            stdin_redirect_path: String::new(),
            stdout_redirect_path: String::new(),
            stderr_redirect_path: String::new(),
            stdin_pipe: None,
            stdout_pipe: None,
            stderr_pipe: None,
        }
    }

    pub fn print_prompt() {
        print!("$ ");
    }

    pub fn get_raw_command() -> io::Result<String> {
        let mut buffer = String::new();
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_string())
    }

    pub fn stdin(&self) -> io::Result<String> {
        match self.stdin_mode {
            IOMode::FILE => {
                let mut file = File::open(&self.stdin_redirect_path)?;
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)?;
                Ok(buffer)
            }
            IOMode::INHERIT => {
                let mut buffer = String::new();
                io::stdout().flush()?;
                io::stdin().read_line(&mut buffer)?;
                Ok(buffer.trim().to_string())
            }
            IOMode::PIPED => Ok(String::new()),
            IOMode::NULL => Ok(String::new()),
        }
    }

    pub fn stdout(&self, args: fmt::Arguments) {
        println!(
            "current mode: {:?}, path: {:?}",
            self.stdout_mode, self.stdout_redirect_path
        );
        match self.stdout_mode {
            IOMode::FILE => {
                let file_result = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&self.stdout_redirect_path);

                println!("File result: {:?}", file_result);

                if let Ok(mut file) = file_result {
                    let _ = file.write_fmt(args);
                    let _ = writeln!(file);
                } else if let Err(e) = file_result {
                    println!("{}", e);
                    eprintln!(
                        "[ShellIO Error] Unable to write to file: {}",
                        self.stdout_redirect_path
                    );
                }
            }
            IOMode::INHERIT => {
                let _ = io::stdout().write_fmt(args);
                println!();
            }
            IOMode::PIPED => {}
            IOMode::NULL => {}
        }
    }

    pub fn stderr(&self, args: fmt::Arguments) {
        println!(
            "current stderr : {:?} path : {:?}",
            self.stderr_mode, self.stderr_redirect_path
        );
        match self.stderr_mode {
            IOMode::FILE => {
                let file_result = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&self.stderr_redirect_path);
                if let Ok(mut file) = file_result {
                    let _ = file.write_fmt(args);
                    let _ = writeln!(file);
                } else {
                    eprintln!(
                        "[ShellIO Error] Unable to write to file: {}",
                        self.stderr_redirect_path
                    )
                }
            }
            IOMode::INHERIT => {
                let _ = io::stderr().write_fmt(args);
                eprintln!();
            }
            IOMode::PIPED => {}
            IOMode::NULL => {}
        }
    }

    pub fn reset(&mut self) {
        self.stdin_mode = IOMode::INHERIT;
        self.stdout_mode = IOMode::INHERIT;
        self.stderr_mode = IOMode::INHERIT;

        self.stdin_redirect_path.clear();
        self.stdout_redirect_path.clear();
        self.stderr_redirect_path.clear();

        self.stdin_pipe = None;
        self.stdout_pipe = None;
        self.stderr_pipe = None;
    }

    pub fn debug(args: fmt::Arguments) {
        Self::_out(OutLevel::DEBUG, args);
    }
    pub fn info(args: fmt::Arguments) {
        Self::_out(OutLevel::INFO, args);
    }
    pub fn warn(args: fmt::Arguments) {
        Self::_out(OutLevel::WARN, args);
    }
    pub fn error(args: fmt::Arguments) {
        Self::_out(OutLevel::ERROR, args);
    }

    fn _out(level: OutLevel, args: fmt::Arguments) {
        if level < Self::OUT_LEVEL {
            let _ = io::stdout().write_fmt(args);
            println!();
        }
    }
}
