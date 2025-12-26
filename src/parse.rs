use std::env;

pub type Command = String;
pub type Arg = String;
pub type Args = Vec<String>;
pub type EnvPath = Vec<String>;

pub struct ParsedCommand {
    pub command: Command,
    pub args: Args,
}

pub fn parse(raw_command: &mut String) -> Option<ParsedCommand> {
    let v_command: Vec<&str> = raw_command.trim().splitn(2, " ").collect();
    let command = String::from(v_command[0]);
    let arg_str = v_command[1];

    let arg_string = String::from(arg_str);
    let mut args: Vec<String> = Vec::new();
    let mut in_single_quote = false;
    let mut has_token_start = false;
    let mut current_token = String::new();
    for ch in arg_string.chars() {
        match ch {
            '\'' => {
                in_single_quote = !in_single_quote;
                has_token_start = true;
            }
            ch if ch.is_whitespace() => {
                if in_single_quote {
                    current_token.push(ch);
                } else {
                    if has_token_start {
                        if !current_token.is_empty() {
                            args.push(current_token);
                        }
                        current_token = String::new();
                        has_token_start = false;
                    }
                }
            }
            _ => {
                current_token.push(ch);
                has_token_start = true;
            }
        }
    }
    if !current_token.is_empty() {
        args.push(current_token);
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
