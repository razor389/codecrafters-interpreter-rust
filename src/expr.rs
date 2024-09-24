use crate::token::Token;
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum LiteralValue {
    StringLiteral(String),
    NumberLiteral(f64), // f64 can handle both integers and floats
    BooleanLiteral(bool),
    Nil,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
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
    Variable(Token),
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
            Expr::Literal(literal) => match literal {
                LiteralValue::StringLiteral(s) => write!(f, "{}", s),
                LiteralValue::NumberLiteral(n) => {
                    // Print number literals as floats
                    if n.fract() == 0.0 {
                        // Whole number, format with `.0`
                        write!(f, "{:.1}", n)
                    } else {
                        // Floating-point number, print with full precision
                        write!(f, "{}", n)
                    }
                }
                LiteralValue::BooleanLiteral(b) => write!(f, "{}", b),
                LiteralValue::Nil => write!(f, "nil"),
            },
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator.lexeme, right)
            }
            // Handle variable expressions like `print baz;`
            Expr::Variable(token) => {
                write!(f, "{}", token.lexeme)
            }
            // Handle assignment expressions
            Expr::Assign { name, value } => {
                write!(f, "(assign {} = {})", name.lexeme, value)
            }
        }
    }
}
