use std::{fmt::Display, process::exit};
#[allow(non_camel_case_types)]
#[derive(Debug)]
enum TokenType {
    // Parenthesizes
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,

    STAR,
    DOT,
    SEMICOLON,
    COMMA,
    // Arithmetics
    PLUS,
    MINUS,
    SLASH,
    EQUAL,
    EQUAL_EQUAL,

    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
}
impl Token {
    pub fn new(lexeme_index: usize, content: &[char]) -> Option<Token> {
        let token_type;
        let mut lexeme = content[lexeme_index].to_string();
        let mut literal = String::from("null");
        match lexeme.as_str() {
            "(" => token_type = TokenType::LEFT_PAREN,
            ")" => token_type = TokenType::RIGHT_PAREN,
            "{" => token_type = TokenType::LEFT_BRACE,
            "}" => token_type = TokenType::RIGHT_BRACE,
            "," => token_type = TokenType::COMMA,
            "." => token_type = TokenType::DOT,
            "-" => token_type = TokenType::MINUS,
            "+" => token_type = TokenType::PLUS,
            ";" => token_type = TokenType::SEMICOLON,
            "/" => token_type = TokenType::SLASH,
            "*" => token_type = TokenType::STAR,
            "=" => {
                if let Some(res) = scan_operator(lexeme_index, content) {
                    token_type = res.0;
                    lexeme = res.1.to_string();
                } else {
                    token_type = TokenType::EQUAL;
                }
            }
            _ => return None,
        }
        Some(Token {
            token_type: token_type,
            lexeme: lexeme,
            literal: literal,
        })
    }
    pub fn eof() -> Token {
        Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: "null".to_string(),
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

pub fn scanning(content: &String) -> (Vec<Token>, i32) {
    let mut exit_code = 0;
    let mut tokens: Vec<Token> = vec![];
    let mut line = 1;
    let vec_chars: Vec<char> = content.chars().collect();
    for i in 0..vec_chars.len() {
        match Token::new(i, &vec_chars) {
            Some(token) => tokens.push(token),
            None => match &vec_chars[i] {
                '\n' => line += 1,
                _ => {
                    eprintln!(
                        "[line {line}] Error: Unexpected character: {char}",
                        char = vec_chars[i]
                    );
                    exit_code = 65;
                }
            },
        }
    }
    tokens.push(Token::eof());
    (tokens, exit_code)
}

fn scan_operator(char_index: usize, content: &[char]) -> Option<(TokenType, &str)> {
    match content[char_index] {
        '=' => match content.get(char_index + 1) {
            Some('=') => Some((TokenType::EQUAL_EQUAL, "==")),
            _ => return None,
        },
        _ => None,
    }
}
