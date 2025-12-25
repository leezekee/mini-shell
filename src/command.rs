use crate::parse::ParsedCommand;

const BUILT_IN_COMMANDS: [&str; 3] = ["exit", "type", "echo"];

pub fn not_found(parsed_command: ParsedCommand) {
    println!("{}: command not found", parsed_command.command);
}

pub fn echo(parsed_command: ParsedCommand) {
    println!("{}", parsed_command.args.join(" "));
}

pub fn _type(parsed_command: ParsedCommand) {
    if BUILT_IN_COMMANDS.contains(&parsed_command.as_ref()) {
        println!("{} is a shell builtin", parsed_command.command)
    } else {
        println!("{}: not found", parsed_command.command)
    }
}
