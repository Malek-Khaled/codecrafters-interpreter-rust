#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process::exit;
use token::scanning;

mod token;
fn main() {
    let mut exit_code = 0;
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        exit(exit_code);
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let (tokens, exit_code_scan) = scanning(&file_contents);
            exit_code = exit_code_scan;
            for token in tokens {
                println!("{}", token);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
    exit(exit_code);
}
