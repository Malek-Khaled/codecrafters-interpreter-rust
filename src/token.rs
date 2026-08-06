use std::fmt::Display;
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

    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
}
impl Token {
    pub fn new(lexeme: &str) -> Option<Token> {
        let token_type;
        let mut literal = String::from("null");
        match lexeme {
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
            _ => return None,
        }
        Some(Token {
            token_type: token_type,
            lexeme: lexeme.to_string(),
            literal: literal,
        })
    }
    pub fn eof_token() -> Token {
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
pub fn scanning(content: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut line = 0;
    for char in content.chars() {
        if let Some(token) = Token::new(&char.to_string()) {
            tokens.push(token);
        } else if char == '\n' {
            line += 1;
        } else {
            eprintln!("[line {line}] Error: Unexpected character:{char}");
        }
    }
    tokens.push(Token::eof_token());
    tokens
}
