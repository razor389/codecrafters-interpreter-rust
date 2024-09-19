// token.rs
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
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
    
    // keywords
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
