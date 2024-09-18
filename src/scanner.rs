use crate::token::{Token, TokenType};
use log::{debug, info};  // Import log macros

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
        info!("Initializing scanner with source of length: {}", source.len());
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
        info!("Reached end of file. Adding EOF token.");
        self.tokens.push(Token::new(TokenType::EOF, String::new(), None));
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        debug!("Scanning token at line {}, character: '{}'", self.line, c);
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
            '/' =>{
                if self.match_next('/') {
                    self.skip_to_end_of_line();
                    debug!("Skipped comment to end of line.");
                }
                else{
                    self.add_token(TokenType::SLASH);
                }
            }
            '\n' => {
                self.line += 1;
                debug!("New line encountered. Line number now: {}", self.line);
            }
            '\0' => (), // Do nothing for null character (end of file)
            _ => self.error(c),  // Handle unknown characters or errors
        }
    }

    /// Advances and returns the next character, or '\0' if at the end.
    fn advance(&mut self) -> char {
        if let Some(c) = self.source.chars().nth(self.current) {
            self.current += 1;
            c
        } else {
            '\0' // Null character to signify the end of the input
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        debug!("Adding token: {:?}, lexeme: {}", token_type, text);
        self.tokens.push(Token::new(token_type, text, None));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if let Some(next_char) = self.source.chars().nth(self.current) {
            if next_char == expected {
                self.current += 1; // Consume the next character
                return true;
            }
        }
        false
    }

    // Skip the rest of the line when encountering `//`
    fn skip_to_end_of_line(&mut self) {
        while let Some(c) = self.source.chars().nth(self.current) {
            debug!("Looping here");
            if c == '\n' {
                break;
            }
            self.current += 1; // Advance to the next character
        }
    }

    // Error reporting function for unexpected characters
    fn error(&mut self, unexpected_char: char) {
        eprintln!("[line {}] Error: Unexpected character: '{}'", self.line, unexpected_char);
        self.error_occurred = true;
    }

    pub fn has_error(&self) -> bool {
        self.error_occurred
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}
