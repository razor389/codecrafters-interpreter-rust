use std::collections::HashMap;

use crate::token::{Token, TokenType};
use log::{debug, info};  // Import log macros

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize, // Track the current line number
    error_occurred: bool,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), TokenType::AND);
        keywords.insert("class".to_string(), TokenType::CLASS);
        keywords.insert("else".to_string(), TokenType::ELSE);
        keywords.insert("false".to_string(), TokenType::FALSE);
        keywords.insert("for".to_string(), TokenType::FOR);
        keywords.insert("fun".to_string(), TokenType::FUN);
        keywords.insert("if".to_string(), TokenType::IF);
        keywords.insert("nil".to_string(), TokenType::NIL);
        keywords.insert("or".to_string(), TokenType::OR);
        keywords.insert("print".to_string(), TokenType::PRINT);
        keywords.insert("return".to_string(), TokenType::RETURN);
        keywords.insert("super".to_string(), TokenType::SUPER);
        keywords.insert("this".to_string(), TokenType::THIS);
        keywords.insert("true".to_string(), TokenType::TRUE);
        keywords.insert("var".to_string(), TokenType::VAR);
        keywords.insert("while".to_string(), TokenType::WHILE);

        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            error_occurred: false,
            keywords,  // Initialize the keywords map
        }
    }

    /// Main loop for scanning tokens.
    pub fn scan_tokens(&mut self) {
        // Continue scanning tokens until scan_token returns None
        while self.scan_token().is_some() {}
        info!("Reached end of file. Adding EOF token.");
        self.tokens.push(Token::new(TokenType::EOF, String::new(), None, self.line));
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
                } 
                else if c.is_alphabetic() || c == '_' {
                    self.scan_identifier(); // Handle identifiers and reserved words
                }
                else {
                    let error_msg = format!("Unexpected character: {}", c);
                    self.error_message(&error_msg);
                }
            } // Handle unknown characters or errors
        }

        Some(())
    }

    /// Advances and returns the next character, or `None` if at the end.
    fn advance(&mut self) -> Option<char> {
        let mut chars = self.source[self.current..].chars();
        let next_char = chars.next();
        if let Some(c) = next_char {
            self.current += c.len_utf8(); // Correctly advance by character's byte length
        }
        next_char
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        debug!("Adding token: {:?}, lexeme: {}", token_type, text);
        self.tokens.push(Token::new(token_type, text, None, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let mut chars = self.source[self.current..].chars();
        if let Some(next_char) = chars.next() {
            if next_char == expected {
                self.current += next_char.len_utf8(); // Correctly advance by the character's byte length
                return true;
            }
        }
        false
    }


    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        debug!("Adding token with literal: {:?}, lexeme: {}, literal: {:?}", token_type, text, literal);
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

    /// Scan an identifier or reserved word
    fn scan_identifier(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        // Extract the identifier lexeme
        let lexeme = self.source[self.start..self.current].to_string();

        // Check if it's a reserved word
        if let Some(token_type) = self.keywords.get(&lexeme) {
            self.add_token(token_type.clone());
        } else {
            self.add_token(TokenType::IDENTIFIER);
        }
    }

    /// Scan number literals (integer or float)
    fn scan_number(&mut self) {
        // Consume digits for the integer part
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }
    
        // Check if there's a fractional part (e.g., 1234.5678)
        let mut has_fractional_part = false;
        if let Some('.') = self.peek() {
            if let Some(next) = self.peek_next() {
                if next.is_digit(10) {
                    self.advance(); // Consume the '.'
                    has_fractional_part = true;
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
    
        // Extract the lexeme
        let lexeme = self.source[self.start..self.current].to_string();
    
        // Parse the literal value as f64
        let literal_value: f64 = lexeme.parse::<f64>().unwrap();
    
        // If there is a fractional part, check if it's all zeros
        let literal_str = if has_fractional_part && lexeme.contains('.') {
            let parts: Vec<&str> = lexeme.split('.').collect();
            let fractional_part = parts[1];
    
            // If the fractional part is all zeros, treat it as an integer (e.g., "200.00" -> "200.0")
            if fractional_part.chars().all(|c| c == '0') {
                format!("{:.1}", literal_value)  // Format as "x.0"
            } else {
                literal_value.to_string()  // Otherwise, keep the full precision
            }
        } else {
            // If there's no fractional part, it's an integer
            format!("{:.1}", literal_value)
        };
    
        // Add the token with the original lexeme and formatted literal
        self.add_token_with_literal(TokenType::NUMBER, Some(literal_str));
    }
    
    /// Peek at the current character without advancing
    fn peek(&self) -> Option<char> {
        self.source[self.current..].chars().next()
    }

    /// Peek at the next character (after the current one) without advancing
    fn peek_next(&self) -> Option<char> {
        let mut chars = self.source[self.current..].chars();
        chars.next()?;
        chars.next()
    }
    
    /// Scan string literals and handle unterminated strings
    fn scan_string(&mut self) {
        while let Some(c) = self.advance() {
            if c == '"' {
                // Closing quote found, add the string token
                let value_with_quotes = self.source[self.start.. self.current].to_string();
                let value_without_quotes = self.source[self.start + 1..self.current - 1].to_string(); // Exclude quotes
                debug!("Adding string token, lexeme: {}, literal: {}", value_with_quotes.clone(), value_without_quotes.clone());
                self.tokens.push(Token::new(
                    TokenType::STRING,
                    value_with_quotes.clone(),    // Lexeme (string with quotes)
                    Some(value_without_quotes),  // Literal value (the actual string content)
                    self.line,     
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
