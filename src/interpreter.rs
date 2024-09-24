use crate::expr::{Expr, LiteralValue};
use crate::token::Token;
use std::fmt;
use std::error::Error;

// Define a RuntimeError type for handling errors during expression evaluation
#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub line: usize, // Add line number to error
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n [line {}]", self.message, self.line)
    }
}
impl Error for RuntimeError {}

// Interpreter struct to evaluate expressions
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    // Evaluate the given expression and return a result as a String or error
    pub fn evaluate(&self, expr: &Expr) -> Result<String, RuntimeError> {
        match expr {
            Expr::Literal(value) => self.visit_literal(value),
            Expr::Unary { operator, right } => self.visit_unary(operator, right),
            Expr::Binary { left, operator, right } => self.visit_binary(left, operator, right),
            Expr::Grouping(expr) => self.visit_grouping(expr),
        }
    }

    fn visit_literal(&self, value: &LiteralValue) -> Result<String, RuntimeError> {
        match value {
            LiteralValue::StringLiteral(s) => Ok(s.clone()), // Return string as is
            LiteralValue::NumberLiteral(n) => {
                if n.fract() == 0.0 {
                    // It's an integer, format without a decimal point
                    Ok(format!("{}", *n as i64))
                } else {
                    // It's a float, format it normally
                    Ok(format!("{}", n))
                }
            }
            LiteralValue::BooleanLiteral(b) => Ok(b.to_string()), // Return "true" or "false"
            LiteralValue::Nil => Ok("nil".to_string()), // Return "nil"
        }
    }

    fn visit_grouping(&self, expr: &Expr) -> Result<String, RuntimeError> {
        self.evaluate(expr)
    }

    fn visit_unary(&self, operator: &Token, right: &Expr) -> Result<String, RuntimeError> {
        let right_value = self.evaluate(right)?;
        match operator.token_type {
            crate::token::TokenType::MINUS => {
                let number = self.expect_number_literal(right, operator.line)?;
                Ok((-number).to_string())
            }
            crate::token::TokenType::BANG => Ok((!self.is_truthy(&right_value)).to_string()),
            _ => Err(RuntimeError {
                message: format!("Unknown unary operator: {}", operator.lexeme),
                line: operator.line,
            }),
        }
    }

    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> Result<String, RuntimeError> {
        let left_value = self.evaluate(left)?;
        let right_value = self.evaluate(right)?;
    
        match operator.token_type {
            crate::token::TokenType::PLUS => {
                // Handle string concatenation or numeric addition
                if let (Ok(left_str), Ok(right_str)) = (self.try_as_string_literal(left), self.try_as_string_literal(right)) {
                    return Ok(left_str + &right_str); // Concatenate strings
                }
    
                if let (Ok(left_num), Ok(right_num)) = (self.try_as_number_literal(left), self.try_as_number_literal(right)) {
                    return Ok((left_num + right_num).to_string()); // Add numbers
                }
    
                Err(RuntimeError {
                    message: "Operands must be two numbers or two strings.".to_string(),
                    line: operator.line,
                })
            }
            crate::token::TokenType::MINUS => {
                let left_num = self.expect_number_literal(left, operator.line)?;
                let right_num = self.expect_number_literal(right, operator.line)?;
                Ok((left_num - right_num).to_string()) // Subtract numbers
            }
            crate::token::TokenType::STAR => {
                let left_num = self.expect_number_literal(left, operator.line)?;
                let right_num = self.expect_number_literal(right, operator.line)?;
                Ok((left_num * right_num).to_string()) // Multiply numbers
            }
            crate::token::TokenType::SLASH => {
                let left_num = self.expect_number_literal(left, operator.line)?;
                let right_num = self.expect_number_literal(right, operator.line)?;
                if right_num == 0.0 {
                    return Err(RuntimeError {
                        message: "Division by zero.".to_string(),
                        line: operator.line,
                    });
                }
                Ok((left_num / right_num).to_string()) // Divide numbers
            }
            // Handle relational operators
            crate::token::TokenType::GREATER => {
                let left_num = self.expect_number_literal(left, operator.line)?;
                let right_num = self.expect_number_literal(right, operator.line)?;
                Ok((left_num > right_num).to_string()) // Greater than
            }
            crate::token::TokenType::GREATER_EQUAL => {
                let left_num = self.expect_number_literal(left, operator.line)?;
                let right_num = self.expect_number_literal(right, operator.line)?;
                Ok((left_num >= right_num).to_string()) // Greater than or equal to
            }
            crate::token::TokenType::LESS => {
                let left_num = self.expect_number_literal(left, operator.line)?;
                let right_num = self.expect_number_literal(right, operator.line)?;
                Ok((left_num < right_num).to_string()) // Less than
            }
            crate::token::TokenType::LESS_EQUAL => {
                let left_num = self.expect_number_literal(left, operator.line)?;
                let right_num = self.expect_number_literal(right, operator.line)?;
                Ok((left_num <= right_num).to_string()) // Less than or equal to
            }
            crate::token::TokenType::EQUAL_EQUAL => {
                // Equality check: works for both strings and numbers
                Ok((left_value == right_value).to_string())
            }
            crate::token::TokenType::BANG_EQUAL => {
                // Inequality check: works for both strings and numbers
                Ok((left_value != right_value).to_string())
            }
            _ => Err(RuntimeError {
                message: format!("Unknown operator: {}", operator.lexeme),
                line: operator.line,
            }),
        }
    }
    
     // Helper function to extract number literals from expressions
     fn expect_number_literal(&self, expr: &Expr, line: usize) -> Result<f64, RuntimeError> {
        if let Expr::Literal(LiteralValue::NumberLiteral(n)) = expr {
            Ok(*n)
        } else {
            Err(RuntimeError {
                message: "Operand must be a number.".to_string(),
                line,
            })
        }
    }

    // Helper function to try and extract number literals
    fn try_as_number_literal(&self, expr: &Expr) -> Result<f64, RuntimeError> {
        if let Expr::Literal(LiteralValue::NumberLiteral(n)) = expr {
            Ok(*n)
        } else {
            Err(RuntimeError {
                message: "Expected a number.".to_string(),
                line: 0,
            })
        }
    }

    // Helper function to try and extract string literals
    fn try_as_string_literal(&self, expr: &Expr) -> Result<String, RuntimeError> {
        if let Expr::Literal(LiteralValue::StringLiteral(s)) = expr {
            Ok(s.clone())
        } else {
            Err(RuntimeError {
                message: "Expected a string.".to_string(),
                line: 0,
            })
        }
    }

    // Helper method to determine if a value is "truthy"
    fn is_truthy(&self, value: &str) -> bool {
        !matches!(value, "false" | "nil")
    }
}
