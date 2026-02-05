
use std::{io::{self, Write}};

use crate::process_executer::execute_process;

pub mod process_executer;

fn main() {
    loop {
        print!("rust_shell> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        let mut prompt = input.split_whitespace();
        
        let command = prompt.next();
        let args: Vec<&str> = prompt.collect();

        match command {
            None => continue,
            Some(exe) =>{
                if exe == "exit" {
                    break; 
                }
                match execute_process(exe, args) {
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