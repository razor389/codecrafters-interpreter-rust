use crate::token::Token;
use std::fmt;

pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(LiteralValue),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Binary { left, operator, right } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Grouping(expr) => {
                write!(f, "(group {})", expr)
            }
            Expr::Literal(value) => match value {
                LiteralValue::Number(n) => write!(f, "{}", n),
                LiteralValue::String(s) => write!(f, "\"{}\"", s),
                LiteralValue::Boolean(b) => write!(f, "{}", b),
                LiteralValue::Nil => write!(f, "nil"),
            },
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator.lexeme, right)
            }
        }
    }
}
