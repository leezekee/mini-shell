use crate::parse::{self, ParsedCommand};
use std::fs;
use std::path::PathBuf;

const BUILT_IN_COMMANDS: [&str; 3] = ["exit", "type", "echo"];

pub fn not_found(parsed_command: ParsedCommand) {
    println!("{}: command not found", parsed_command.command);
}

pub fn echo(parsed_command: ParsedCommand) {
    println!("{}", parsed_command.args.join(" "));
}

pub fn _type(parsed_command: ParsedCommand) {
    let mut command = parsed_command.args.join(" ");
    let paths = parse::get_env_path();
    if BUILT_IN_COMMANDS.contains(&command.as_ref()) {
        println!("{} is a shell builtin", command)
    } else {
        go_through_paths(&paths);
        // search arguments in paths
        match search_file_in_paths(command.as_mut_str(), &paths) {
            Some(path) => println!("{} is {}", command, path.display()),
            None => println!("{}: not found", command),
        }
    }
}

fn search_file_in_paths(filename: &str, paths: &[&str]) -> Option<PathBuf> {
    paths.iter().find_map(|dir| {
        let full_path = PathBuf::from(dir).join(filename);
        if full_path.is_file() {
            Some(full_path)
        } else {
            None
        }
    })
}

fn go_through_paths(paths: &[&str]) {
    for path_str in paths {
        // 2. 筛选：只处理以 "/tmp" 开头的路径
        if path_str.starts_with("/tmp") {
            println!("Found directory: [{}]", path_str);

            // 3. 读取目录内容
            // fs::read_dir 可能会失败（比如没权限，或者文件夹不存在），所以要处理 Result
            match fs::read_dir(path_str) {
                Ok(entries) => {
                    for entry in entries {
                        match entry {
                            Ok(dir_entry) => {
                                // 获取文件名
                                let file_name = dir_entry.file_name();
                                // 转换为字符串以便打印 (lossy转换处理特殊字符)
                                println!("  └── {}", file_name.to_string_lossy());
                            }
                            Err(e) => println!("  └── [Error reading entry: {}]", e),
                        }
                    }
                }
                Err(e) => {
                    println!("  └── [Cannot read directory: {}]", e);
                }
            }
            println!("--------------------------------");
        }
    }
}
