use crate::expr::Expr;
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

    fn visit_literal(&self, value: &Option<String>) -> Result<String, RuntimeError> {
        match value {
            Some(v) => {
                // Try to parse the value as a float or integer and format accordingly
                if let Ok(num) = v.parse::<f64>() {
                    if num.fract() == 0.0 {
                        // It's an integer, format without a decimal point
                        Ok(format!("{}", num as i64))
                    } else {
                        // It's a float, format it normally
                        Ok(format!("{}", num))
                    }
                } else {
                    // It's a string, return it directly
                    Ok(v.clone())
                }
            }
            None => Ok("nil".to_string()),
        }
    }

    fn visit_grouping(&self, expr: &Expr) -> Result<String, RuntimeError> {
        self.evaluate(expr)
    }

    fn visit_unary(&self, operator: &Token, right: &Expr) -> Result<String, RuntimeError> {
        let right_value = self.evaluate(right)?;
        match operator.token_type {
            // Handle negation for numeric values
            crate::token::TokenType::MINUS => {
                let number = right_value.parse::<f64>().map_err(|_| RuntimeError {
                    message: "Operand must be a number.".to_string(),
                    line: operator.line,
                })?;
                Ok((-number).to_string())
            }
            // Handle logical NOT for booleans
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
                let left_is_num = left_value.parse::<f64>().is_ok();
                let right_is_num = right_value.parse::<f64>().is_ok();

                if left_is_num && right_is_num {
                    let left_num = left_value.parse::<f64>().unwrap();
                    let right_num = right_value.parse::<f64>().unwrap();
                    Ok((left_num + right_num).to_string())
                } else if !left_is_num && !right_is_num {
                    Ok(left_value + &right_value) // String concatenation
                } else {
                    Err(RuntimeError {
                        message: "Operands must be two numbers or two strings.".to_string(),
                        line: operator.line,
                    })
                }
            }
            crate::token::TokenType::MINUS
            | crate::token::TokenType::STAR
            | crate::token::TokenType::SLASH => {
                self.check_numeric_operands(&left_value, &right_value, operator.line)?;
                let left_num = left_value.parse::<f64>().unwrap();
                let right_num = right_value.parse::<f64>().unwrap();

                match operator.token_type {
                    crate::token::TokenType::MINUS => Ok((left_num - right_num).to_string()),
                    crate::token::TokenType::STAR => Ok((left_num * right_num).to_string()),
                    crate::token::TokenType::SLASH => {
                        if right_num == 0.0 {
                            return Err(RuntimeError {
                                message: "Division by zero.".to_string(),
                                line: operator.line,
                            });
                        }
                        Ok((left_num / right_num).to_string())
                    }
                    _ => unreachable!(),
                }
            }
            // Relational operators
            crate::token::TokenType::GREATER
            | crate::token::TokenType::GREATER_EQUAL
            | crate::token::TokenType::LESS
            | crate::token::TokenType::LESS_EQUAL => {
                self.check_numeric_operands(&left_value, &right_value, operator.line)?;
                let left_num = left_value.parse::<f64>().unwrap();
                let right_num = right_value.parse::<f64>().unwrap();

                match operator.token_type {
                    crate::token::TokenType::GREATER => Ok((left_num > right_num).to_string()),
                    crate::token::TokenType::GREATER_EQUAL => Ok((left_num >= right_num).to_string()),
                    crate::token::TokenType::LESS => Ok((left_num < right_num).to_string()),
                    crate::token::TokenType::LESS_EQUAL => Ok((left_num <= right_num).to_string()),
                    _ => unreachable!(),
                }
            }
            // Equality operators
            crate::token::TokenType::EQUAL_EQUAL => Ok((left_value == right_value).to_string()),
            crate::token::TokenType::BANG_EQUAL => Ok((left_value != right_value).to_string()),
            _ => Err(RuntimeError {
                message: format!("Unknown binary operator: {}", operator.lexeme),
                line: operator.line,
            }),
        }
    }

    // Helper method to check if both operands are valid numbers for arithmetic operations
    fn check_numeric_operands(&self, left: &str, right: &str, line: usize) -> Result<(), RuntimeError> {
        if left.parse::<f64>().is_err() || right.parse::<f64>().is_err() {
            return Err(RuntimeError {
                message: "Operands must be numbers.".to_string(),
                line,
            });
        }
        Ok(())
    }

    // Helper method to determine if a value is "truthy"
    fn is_truthy(&self, value: &str) -> bool {
        !matches!(value, "false" | "nil")
    }
}
