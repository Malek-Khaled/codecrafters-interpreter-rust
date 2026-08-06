use std::fmt::Display;
#[allow(non_camel_case_types)]
#[derive(Debug)]
enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
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
pub fn tokenizing(content: &String) -> Vec<Token> {
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
