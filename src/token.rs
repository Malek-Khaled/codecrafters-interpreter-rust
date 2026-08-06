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
