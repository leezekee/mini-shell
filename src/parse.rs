use std::{cell::RefCell, collections::HashMap, env, fmt::Display, rc::Rc, str::FromStr};

use crate::{
    command,
    error::ShellError,
    utils::{execute_external, search_file_in_paths},
};

pub type Command = String;
pub type Arg = String;
pub type Args = Vec<String>;
pub type EnvPath = Vec<String>;
pub type Handler = fn(ParsedCommand, RunTimeEnvPath) -> ShellResult;
pub type RunTimeEnvPath = Rc<RefCell<EnvPath>>;
pub type ShellResult = Result<i32, ShellError>;

#[derive(Clone)]
pub struct ParsedCommand {
    pub command: Command,
    pub args: Args,
}

#[derive(PartialEq)]
pub enum ParseMode {
    SingleQuote,
    DoubleQuote,
    None,
}

const SINGLE_QUOTE: char = '\'';
const DOUBLE_QUOTE: char = '\"';

pub fn parse(raw_command: &mut String) -> Option<ParsedCommand> {
    let v_command: Vec<&str> = raw_command.trim().splitn(2, " ").collect();
    let command = String::from(v_command[0]);

    // no arguments
    if v_command.len() == 1 {
        let args: Args = Vec::new();
        return Some(ParsedCommand { command, args });
    }

    let mut args: Vec<String> = Vec::new();
    let mut current_token = String::new();
    let mut mode = ParseMode::None;
    for ch in v_command[1].chars() {
        match ch {
            SINGLE_QUOTE => match mode {
                ParseMode::SingleQuote => mode = ParseMode::None,
                ParseMode::None => mode = ParseMode::SingleQuote,
                _ => current_token.push(ch),
            },
            DOUBLE_QUOTE => match mode {
                ParseMode::DoubleQuote => mode = ParseMode::None,
                _ => mode = ParseMode::DoubleQuote,
            },
            _ => {
                if ch.is_whitespace() && mode == ParseMode::None {
                    if !current_token.is_empty() {
                        args.push(std::mem::take(&mut current_token));
                    }
                } else {
                    current_token.push(ch);
                }
            }
        }
    }

    if !current_token.is_empty() && mode != ParseMode::None {
        return None;
    }

    Some(ParsedCommand { command, args })
}

impl<'a> PartialEq<&str> for ParsedCommand {
    fn eq(&self, target: &&str) -> bool {
        self.command == *target
    }
}

impl<'a> PartialEq<ParsedCommand> for &'a str {
    fn eq(&self, parsed_command: &ParsedCommand) -> bool {
        *self == parsed_command.command
    }
}

impl<'a> AsRef<str> for ParsedCommand {
    fn as_ref(&self) -> &str {
        &self.command
    }
}

pub fn get_env_path() -> EnvPath {
    let path_string = env::var("PATH").unwrap_or_default();
    let env_path: EnvPath = path_string
        .split(if cfg!(windows) { ';' } else { ':' })
        .filter(|&p| !p.is_empty() && p != "$PATH")
        .map(|p| String::from(p))
        .collect();

    env_path
}

pub fn get_env_home<'a>() -> String {
    env::var("HOME").unwrap_or_default()
}

pub struct CommandHandler {
    built_in_command: HashMap<BuiltIn, Handler>,
    local_path: EnvPath,
    temp_path: EnvPath,
    runtime_path: RunTimeEnvPath,
}

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
pub enum BuiltIn {
    ECHO,
    EXIT,
    CD,
    PWD,
    TYPE,
}

impl FromStr for BuiltIn {
    type Err = ShellError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "echo" => Ok(BuiltIn::ECHO),
            "exit" => Ok(BuiltIn::EXIT),
            "cd" => Ok(BuiltIn::CD),
            "pwd" => Ok(BuiltIn::PWD),
            "type" => Ok(BuiltIn::TYPE),
            _ => Err(ShellError::NotABuiltinCommand),
        }
    }
}

impl Display for BuiltIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = format!("{:?}", self).to_lowercase();

        write!(f, "{}", output)
    }
}

impl CommandHandler {
    pub fn new() -> CommandHandler {
        let mut command_handler = CommandHandler {
            built_in_command: HashMap::new(),
            local_path: get_env_path(),
            temp_path: Vec::new(),
            runtime_path: Rc::new(RefCell::new(Vec::new())),
        };

        // register command
        command_handler.register(BuiltIn::ECHO, command::echo);
        command_handler.register(BuiltIn::EXIT, command::_exit);
        command_handler.register(BuiltIn::CD, command::cd);
        command_handler.register(BuiltIn::PWD, command::pwd);
        command_handler.register(BuiltIn::TYPE, command::_type);

        command_handler
    }

    fn register(&mut self, command: BuiltIn, handler: Handler) {
        self.built_in_command.insert(command, handler);
    }

    fn get_runtime_path(&self) -> RunTimeEnvPath {
        if self.runtime_path.borrow().is_empty() {
            let mut runtime_path = self.runtime_path.borrow_mut();
            *runtime_path = self
                .local_path
                .iter()
                .chain(self.temp_path.iter())
                .cloned()
                .collect();
        }
        println!("{:?}", self.runtime_path.borrow());
        self.runtime_path.clone()
    }

    fn run_built_in_command(&self, command: BuiltIn, parsed_command: ParsedCommand) -> ShellResult {
        self.built_in_command.get(&command).unwrap()(parsed_command, self.get_runtime_path())
    }

    fn run_external_command(&self, parsed_command: ParsedCommand) -> ShellResult {
        match search_file_in_paths(&parsed_command.command, self.get_runtime_path()) {
            Some(_) => execute_external(&parsed_command.command, parsed_command.args),
            None => return Err(ShellError::CommandNotFound(parsed_command.command)),
        }
    }

    pub fn run(&mut self, parsed_command: ParsedCommand) -> ShellResult {
        let result: ShellResult = match parsed_command.command.parse::<BuiltIn>() {
            Ok(cmd) => self.run_built_in_command(cmd, parsed_command),
            _ => self.run_external_command(parsed_command),
        };
        self.temp_path.clear();
        result
    }
}
