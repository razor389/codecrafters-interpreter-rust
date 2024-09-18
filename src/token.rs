// token.rs
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens
    LEFT_PAREN,    // (
    RIGHT_PAREN,   // )
    LEFT_BRACE,    // {
    RIGHT_BRACE,   // }
    STAR,          // *
    DOT,           // .
    COMMA,         // ,
    PLUS,          // +
    MINUS,         // -
    SEMICOLON,     // ;
    EQUAL,         // =
    EQUAL_EQUAL,   // ==
    BANG,          // !
    BANG_EQUAL,    // !=
    LESS,          // <
    LESS_EQUAL,    // <=
    GREATER,       // >
    GREATER_EQUAL, // >=
    // Add more as we expand the language spec
    
    // End of file
    EOF,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>, // For tokens that have a literal value (not used here)
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
        }
    }
}
