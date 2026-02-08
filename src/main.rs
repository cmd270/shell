use std::env;
use std::io::{self, Write};

pub mod commands;
pub mod process_executer;

const BUILT_IN_COMMANDS: [&str; 6] = ["cd", "pwd", "echo", "clear", "dir", "exit"];

fn main() {
    loop {
        let current_path = env::current_dir().unwrap_or_default();
        print!("RS {} > ", current_path.display());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        let mut prompt = input.split_whitespace();

        let command = prompt.next();
        let args: Vec<&str> = prompt.collect();

        match command {
            None => continue,
            Some(command_str) => {
                if BUILT_IN_COMMANDS.contains(&command_str) {
                    match command_str {
                        "cd" => commands::cd::execute(args),
                        "pwd" => commands::pwd::execute(),
                        "echo" => commands::echo::execute(args),
                        "clear" => commands::clear::execute(),
                        "dir" => commands::dir::execute(args),
                        "exit" => commands::exit::exit(),
                        _ => println!("Whoops.. build-in command is not found."),
                    }
                    continue;
                }

                match process_executer::execute_process(command_str, args) {
                    Ok(output) => {
                        println!("{}", output);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        continue;
                    }
                }
            }
        };
    }
}
