#![allow(unused_variables)]
use std::env;
use std::fmt::Display;
use std::fs;
use token::*;

mod token;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let tokens = tokenizing(&file_contents);
            for token in tokens {
                println!("{}", token);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn tokenizing(content: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    for char in content.chars() {
        match char {
            '(' => tokens.push(Token::new(TokenType::LEFT_PAREN, "(", "null")),
            ')' => tokens.push(Token::new(TokenType::RIGHT_PAREN, ")", "null")),
            '{' => tokens.push(Token::new(TokenType::LEFT_BRACE, "{", "null")),
            '}' => tokens.push(Token::new(TokenType::RIGHT_BRACE, "}", "null")),
            _ => continue,
        }
    }
    tokens.push(Token::new(TokenType::EOF, "", "null"));
    tokens
}
