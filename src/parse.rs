type Command<'a> = &'a str;
type Arg<'a> = &'a str;
type Args<'a> = Vec<&'a str>;

pub struct ParsedCommand<'a> {
    pub command: Command<'a>,
    pub args: Args<'a>,
}

pub fn parse(raw_command: &mut String) -> Option<ParsedCommand> {
    let mut arg_vec: Vec<&str> = raw_command.trim().split_whitespace().collect();

    if arg_vec.is_empty() {
        return None;
    }

    let command = arg_vec.remove(0);
    let args = arg_vec;

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
