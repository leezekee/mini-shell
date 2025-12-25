use crate::parse::ParsedCommand;

pub fn not_found(parsed_command: ParsedCommand) {
    println!("{}: command not found", parsed_command.command);
}

pub fn echo(parsed_command: ParsedCommand) {
    println!("{}", parsed_command.args.join(" "));
}
