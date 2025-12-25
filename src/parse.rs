use std::env;

type Command<'a> = &'a str;
type Arg<'a> = &'a str;
type Args<'a> = Vec<&'a str>;
type EnvPath<'a> = Vec<&'a str>;

pub struct ParsedCommand<'a> {
    pub command: Command<'a>,
    pub args: Args<'a>,
}

pub fn parse<'a>(raw_command: &'a mut String) -> Option<ParsedCommand<'a>> {
    let mut arg_vec: Vec<&str> = raw_command.trim().split_whitespace().collect();

    if arg_vec.is_empty() {
        return None;
    }

    let command: Command = arg_vec.remove(0);
    let args: Args = arg_vec;

    Some(ParsedCommand { command, args })
}

impl<'a> PartialEq<&str> for ParsedCommand<'a> {
    fn eq(&self, target: &&str) -> bool {
        self.command == *target
    }
}

impl<'a> PartialEq<ParsedCommand<'a>> for &'a str {
    fn eq(&self, parsed_command: &ParsedCommand<'a>) -> bool {
        *self == parsed_command.command
    }
}

impl<'a> AsRef<str> for ParsedCommand<'a> {
    fn as_ref(&self) -> &str {
        &self.command
    }
}

pub fn get_env_path<'a>() -> EnvPath<'a> {
    let path_string = env::var("PATH").unwrap_or_default();
    let static_path_str = Box::leak(path_string.into_boxed_str());
    let env_path: EnvPath = static_path_str
        .split(if cfg!(windows) { ';' } else { ':' })
        .filter(|&p| !p.is_empty() && p != "$PATH")
        .collect();

    env_path
}
