use std::fmt;

// token.rs
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
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
    SLASH,         // /

    //literals
    IDENTIFIER,    // identifier (variable name)
    STRING,        // string literal
    NUMBER,        // number literal (integer or float)
    
    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    
    // End of file
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>, 
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

// Implement Display for Token to format it as "<TokenType> <Lexeme> <Literal>"
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let literal_display = match &self.literal {
            Some(lit) => lit.clone(),
            None => "null".to_string(),
        };
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, literal_display)
    }
}