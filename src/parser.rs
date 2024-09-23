use crate::token::{Token, TokenType};
use crate::expr::Expr;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression()
    }

    // expression → equality
    fn expression(&mut self) -> Option<Expr> {
        self.equality()
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )*
    fn equality(&mut self) -> Option<Expr> {
        let mut expr = self.comparison();

        while self.match_token(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Some(Expr::Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right?),
            });
        }

        expr
    }

    // comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )*
    fn comparison(&mut self) -> Option<Expr> {
        let mut expr = self.term();

        while self.match_token(&[
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Some(Expr::Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right?),
            });
        }

        expr
    }

    // term → factor ( ( "-" | "+" ) factor )*
    fn term(&mut self) -> Option<Expr> {
        let mut expr = self.factor();

        while self.match_token(&[TokenType::PLUS, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Some(Expr::Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right?),
            });
        }

        expr
    }

    // factor → unary ( ( "/" | "*" ) unary )*
    fn factor(&mut self) -> Option<Expr> {
        let mut expr = self.unary();

        while self.match_token(&[TokenType::STAR, TokenType::SLASH]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Some(Expr::Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right?),
            });
        }

        expr
    }

    // unary → ( "!" | "-" ) unary | primary
    fn unary(&mut self) -> Option<Expr> {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Some(Expr::Unary {
                operator,
                right: Box::new(right?),
            });
        }

        self.primary()
    }

    // primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"
    fn primary(&mut self) -> Option<Expr> {
        if self.match_token(&[TokenType::NUMBER]) {
            let value = self.previous().literal.clone();
            return Some(Expr::Literal(value));
        }

        if self.match_token(&[TokenType::STRING]) {
            let value = self.previous().literal.clone();
            return Some(Expr::Literal(value));
        }

        if self.match_token(&[TokenType::TRUE]) {
            return Some(Expr::Literal(Some("true".to_string())));
        }

        if self.match_token(&[TokenType::FALSE]) {
            return Some(Expr::Literal(Some("false".to_string())));
        }

        if self.match_token(&[TokenType::NIL]) {
            return Some(Expr::Literal(None));
        }
        if self.match_token(&[TokenType::LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;
            return Some(Expr::Grouping(Box::new(expr?)));
        }

        None
    }

    // Utility methods

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for &token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Option<&Token> {
        if self.check(token_type) {
            return Some(self.advance());
        }

        eprintln!("{}", message);
        None
    }
}
