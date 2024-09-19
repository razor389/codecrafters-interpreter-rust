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

    /// Main loop for scanning tokens.
    pub fn scan_tokens(&mut self) {
        // Continue scanning tokens until scan_token returns None
        while self.scan_token().is_some() {}
        info!("Reached end of file. Adding EOF token.");
        self.tokens.push(Token::new(TokenType::EOF, String::new(), None));
    }

    /// Scans the next token, returning `Some(())` if a token was found, or `None` if end of file is reached.
    fn scan_token(&mut self) -> Option<()> {
        self.start = self.current;  

        let c = self.advance()?;
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
                    self.skip_to_end_of_line(); // Skip comment to the end of the line
                    debug!("Skipped comment to end of line.");
                }
                else{
                    self.add_token(TokenType::SLASH);
                }
            }
            '"' => {
                self.scan_string(); // Handle string literal
            }
            '\n' => {
                self.line += 1;
                debug!("New line encountered. Line number now: {}", self.line);
            }
            '\t' => {
                debug!("Tab character encountered, no action taken");
            }
            ' ' => {
                debug!("Space character encountered, no action taken");
            }
            _ => {
                if c.is_digit(10) {
                    self.scan_number(); // Handle number literals
                } else {
                    let error_msg = format!("Unexpected character: {}", c);
                    self.error_message(&error_msg);
                }
            } // Handle unknown characters or errors
        }

        Some(())
    }

    /// Advances and returns the next character, or `None` if at the end.
    fn advance(&mut self) -> Option<char> {
        let next_char = self.source.chars().nth(self.current);
        if next_char.is_some() {
            self.current += 1; // Move to the next character
        }
        next_char // Return the character or None if at the end
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

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        debug!("Adding token with literal: {:?}, lexeme: {}, literal: {:?}", token_type, text, literal);
        self.tokens.push(Token::new(token_type, text, literal));
    }

    /// Scan number literals (integer or float)
    fn scan_number(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }

        // Check if there's a fractional part (e.g., 1234.5678)
        if let Some('.') = self.peek() {
            if let Some(next) = self.peek_next() {
                if next.is_digit(10) {
                    self.advance(); // Consume the '.'
                    while let Some(c) = self.peek() {
                        if c.is_digit(10) {
                            self.advance(); // Consume the rest of the number
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        // Extract the lexeme and convert to f64
        let lexeme = self.source[self.start..self.current].to_string();
        let literal_value = lexeme.parse::<f64>().unwrap();
        self.add_token_with_literal(TokenType::NUMBER, Some(literal_value.to_string()));
    }

    /// Peek at the current character without advancing
    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    /// Peek at the next character (after the current one) without advancing
    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    /// Scan string literals and handle unterminated strings
    fn scan_string(&mut self) {
        while let Some(c) = self.advance() {
            if c == '"' {
                // Closing quote found, add the string token
                let value_with_quotes = self.source[self.start.. self.current].to_string();
                let value_without_quotes = self.source[self.start + 1..self.current - 1].to_string(); // Exclude quotes
                self.tokens.push(Token::new(
                    TokenType::STRING,
                    value_with_quotes.clone(),    // Lexeme (string with quotes)
                    Some(value_without_quotes),      // Literal value (the actual string content)
                ));
                return;
            } else if c == '\n' {
                self.line += 1; // Handle multi-line strings
            }
        }

        // If we reach here, the string was unterminated
        self.error_message("Unterminated string.");
    }

    // Skip the rest of the line when encountering `//`
    fn skip_to_end_of_line(&mut self) {
        // Continue advancing until we find a newline or reach the end of the input
        while let Some(c) = self.advance() {
            if c == '\n' {
                self.line += 1; // Increment line number
                break; // Stop at the end of the line
            }
        }
    }

    /// Error reporting for specific messages
    fn error_message(&mut self, message: &str) {
        eprintln!("[line {}] Error: {}", self.line, message);
        self.error_occurred = true;
    }

    pub fn has_error(&self) -> bool {
        self.error_occurred
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}
