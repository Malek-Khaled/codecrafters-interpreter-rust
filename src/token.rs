use std::fmt::Display;
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
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
    pub fn new(lexeme: &str) -> Option<Token> {
        let mut literal = String::from("null");
        let token_type = match lexeme {
            "(" => TokenType::LEFT_PAREN,
            ")" => TokenType::RIGHT_PAREN,
            "{" => TokenType::LEFT_BRACE,
            "}" => TokenType::RIGHT_BRACE,
            "," => TokenType::COMMA,
            "." => TokenType::DOT,
            "-" => TokenType::MINUS,
            "+" => TokenType::PLUS,
            ";" => TokenType::SEMICOLON,
            "/" => TokenType::SLASH,
            "*" => TokenType::STAR,
            "==" => TokenType::EQUAL_EQUAL,
            "=" => TokenType::EQUAL,
            _ => return None,
        };
        Some(Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal,
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

pub fn scanning(content: &str) -> (Vec<Token>, i32) {
    let mut exit_code = 0;
    let mut tokens: Vec<Token> = vec![];
    let mut line = 1;
    let vec_chars: Vec<char> = content.chars().collect();
    let mut i = 0;
    while i < vec_chars.len() {
        match Token::new(&match_op(&mut i, &vec_chars)) {
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
        i += 1;
    }
    tokens.push(Token::eof());
    (tokens, exit_code)
}

fn match_op<'a>(char_index: &'a mut usize, content: &'a [char]) -> String {
    let lexeme = content[*char_index];
    match lexeme {
        '=' => match content.get(*char_index + 1) {
            Some('=') => {
                *char_index += 1;
                "==".to_string()
            }
            _ => lexeme.to_string(),
        },
        _ => lexeme.to_string(),
    }
}
