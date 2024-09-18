// scanner.rs
use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize, // Track the current line number
    error_occurred: bool,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1, // Start at line 1
            error_occurred: false,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::EOF, String::new(), None));
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            '*' => self.add_token(TokenType::STAR),
            '.' => self.add_token(TokenType::DOT),
            ',' => self.add_token(TokenType::COMMA),
            '+' => self.add_token(TokenType::PLUS),
            '-' => self.add_token(TokenType::MINUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EQUAL_EQUAL); // Handle ==
                } else {
                    self.add_token(TokenType::EQUAL); // Handle =
                }
            }
            '!' =>{
                if self.match_next('='){
                    self.add_token(TokenType::BANG_EQUAL);
                } else{
                    self.add_token(TokenType::BANG);
                }
            }
            '<' =>{
                if self.match_next('='){
                    self.add_token(TokenType::LESS_EQUAL);
                }
                else{
                    self.add_token(TokenType::LESS);
                }
            }
            '>' =>{
                if self.match_next('='){
                    self.add_token(TokenType::GREATER_EQUAL);
                }
                else{
                    self.add_token(TokenType::GREATER);
                }
            }
            '\n' => self.line += 1, // Handle line breaks
            // Add more token matching cases here
            _ => self.error(c),  // Handle unknown characters or errors
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, None));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // Check if the next character matches the expected character.
    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1; // Consume the next character
        true
    }

    // Error reporting function for unexpected characters
    fn error(&mut self, unexpected_char: char) {
        eprintln!("[line {}] Error: Unexpected character: {}", self.line, unexpected_char);
        self.error_occurred = true;
    }

    pub fn has_error(&self) -> bool {
        self.error_occurred
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}
