#![allow(unused_variables)]
use std::env;
use std::fmt::Display;
use std::fs;

#[allow(non_camel_case_types)]
enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
}
impl Token {
    fn new(token_type: TokenType, lexme: &str, literal: &str) -> Token {
        Token {
            token_type: token_type,
            lexeme: lexme.to_string(),
            literal: literal.to_string(),
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

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

            tokenizer(&file_contents);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn tokenizer(content: &String) {
    let mut tokens: Vec<Token>;
    for char in content.chars() {
        match char {
            '(' => tokens.push(Token::new(TokenType::LEFT_PAREN, "(", "null")),
            ')' => tokens.push(Token::new(TokenType::RIGHT_PAREN, ")", "null")),
            '{' => tokens.push(Token::new(TokenType::LEFT_BRACE, "{", "null")),
            '}' => tokens.push(Token::new(TokenType::RIGHT_BRACE, "}", "null")),
            _ => continue,
        }
    }
    println!("EOF  null");
}
