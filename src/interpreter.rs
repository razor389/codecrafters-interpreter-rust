use crate::expr::{Expr, LiteralValue};
use crate::stmt::Stmt;
use crate::token::Token;
use std::collections::HashMap;
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

// Environment for storing variables
pub struct Environment {
    values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<&LiteralValue, RuntimeError> {
        self.values.get(name).ok_or_else(|| RuntimeError {
            message: format!("Undefined variable '{}'.", name),
            line: 0, // Adjust line handling if necessary
        })
    }
}

// Interpreter struct to evaluate expressions and statements
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in statements {
            self.execute(&stmt)?;
        }
        Ok(())
    }

    // Execute statements
    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;
                println!("{}", self.literal_to_string(value));
                Ok(())
            }
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(())
            }
            Stmt::Var { name, initializer } => {
                let value = if let Some(expr) = initializer {
                    self.evaluate(expr)?
                } else {
                    LiteralValue::Nil
                };
                self.environment.define(name.lexeme.clone(), value);
                Ok(())
            }
        }
    }

    pub fn literal_to_string(&self, value: LiteralValue) -> String {
        match value {
            LiteralValue::StringLiteral(s) => s,
            LiteralValue::NumberLiteral(n) => {
                if n.fract() == 0.0 {
                    format!("{}", n as i64) // If it's an integer, format without decimal point
                } else {
                    n.to_string() // Otherwise, format as a float
                }
            }
            LiteralValue::BooleanLiteral(b) => b.to_string(),
            LiteralValue::Nil => "nil".to_string(),
        }
    }

    // Evaluate the given expression and return a result as a String or error
    pub fn evaluate(&self, expr: &Expr) -> Result<LiteralValue, RuntimeError> {
        match expr {
            Expr::Literal(value) => self.visit_literal(value),
            Expr::Unary { operator, right } => self.visit_unary(operator, right),
            Expr::Binary { left, operator, right } => self.visit_binary(left, operator, right),
            Expr::Grouping(expr) => self.visit_grouping(expr),
        }
    }
    

    fn visit_literal(&self, value: &LiteralValue) -> Result<LiteralValue, RuntimeError> {
        Ok(value.clone()) // Return the literal value as-is
    }

    fn visit_grouping(&self, expr: &Expr) -> Result<LiteralValue, RuntimeError> {
        self.evaluate(expr)
    }
    

    fn visit_unary(&self, operator: &Token, right: &Expr) -> Result<LiteralValue, RuntimeError> {
        let right_value = self.evaluate(right)?;
        match operator.token_type {
            crate::token::TokenType::MINUS => {
                if let LiteralValue::NumberLiteral(number) = right_value {
                    Ok(LiteralValue::NumberLiteral(-number)) // Return negated number
                } else {
                    Err(RuntimeError {
                        message: "Operand must be a number.".to_string(),
                        line: operator.line,
                    })
                }
            }
            crate::token::TokenType::BANG => {
                let truthy_value = self.is_truthy(&right_value);
                Ok(LiteralValue::BooleanLiteral(!truthy_value))
            }
            _ => Err(RuntimeError {
                message: format!("Unknown unary operator: {}", operator.lexeme),
                line: operator.line,
            }),
        }
    }
    

    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> Result<LiteralValue, RuntimeError> {
        let left_value = self.evaluate(left)?;
        let right_value = self.evaluate(right)?;
    
        match operator.token_type {
            crate::token::TokenType::PLUS => {
                // Handle string concatenation
                if let (LiteralValue::StringLiteral(left_str), LiteralValue::StringLiteral(right_str)) = (&left_value, &right_value) {
                    log::debug!("concatenating strings {} and {}", left_str, right_str);
                    return Ok(LiteralValue::StringLiteral(left_str.clone() + right_str));
                }
    
                // Handle numeric addition
                if let (LiteralValue::NumberLiteral(left_num), LiteralValue::NumberLiteral(right_num)) = (&left_value, &right_value) {
                    return Ok(LiteralValue::NumberLiteral(left_num + right_num));
                }
    
                Err(RuntimeError {
                    message: "Operands must be two numbers or two strings.".to_string(),
                    line: operator.line,
                })
            }
            // Handle subtraction, multiplication, and division
            crate::token::TokenType::MINUS => {
                let left_num = self.expect_number_literal(&left_value, operator.line)?;
                let right_num = self.expect_number_literal(&right_value, operator.line)?;
                Ok(LiteralValue::NumberLiteral(left_num - right_num))
            }
            crate::token::TokenType::STAR => {
                let left_num = self.expect_number_literal(&left_value, operator.line)?;
                let right_num = self.expect_number_literal(&right_value, operator.line)?;
                Ok(LiteralValue::NumberLiteral(left_num * right_num))
            }
            crate::token::TokenType::SLASH => {
                let left_num = self.expect_number_literal(&left_value, operator.line)?;
                let right_num = self.expect_number_literal(&right_value, operator.line)?;
                if right_num == 0.0 {
                    return Err(RuntimeError {
                        message: "Division by zero.".to_string(),
                        line: operator.line,
                    });
                }
                Ok(LiteralValue::NumberLiteral(left_num / right_num))
            }
            // Handle relational operators (>, <, >=, <=)
            crate::token::TokenType::GREATER => {
                let left_num = self.expect_number_literal(&left_value, operator.line)?;
                let right_num = self.expect_number_literal(&right_value, operator.line)?;
                Ok(LiteralValue::BooleanLiteral(left_num > right_num))
            }
            crate::token::TokenType::GREATER_EQUAL => {
                let left_num = self.expect_number_literal(&left_value, operator.line)?;
                let right_num = self.expect_number_literal(&right_value, operator.line)?;
                Ok(LiteralValue::BooleanLiteral(left_num >= right_num))
            }
            crate::token::TokenType::LESS => {
                let left_num = self.expect_number_literal(&left_value, operator.line)?;
                let right_num = self.expect_number_literal(&right_value, operator.line)?;
                Ok(LiteralValue::BooleanLiteral(left_num < right_num))
            }
            crate::token::TokenType::LESS_EQUAL => {
                let left_num = self.expect_number_literal(&left_value, operator.line)?;
                let right_num = self.expect_number_literal(&right_value, operator.line)?;
                Ok(LiteralValue::BooleanLiteral(left_num <= right_num))
            }
            // Handle equality
            crate::token::TokenType::EQUAL_EQUAL => Ok(LiteralValue::BooleanLiteral(left_value == right_value)),
            crate::token::TokenType::BANG_EQUAL => Ok(LiteralValue::BooleanLiteral(left_value != right_value)),
            _ => Err(RuntimeError {
                message: format!("Unknown operator: {}", operator.lexeme),
                line: operator.line,
            }),
        }
    }
    
    fn expect_number_literal(&self, value: &LiteralValue, line: usize) -> Result<f64, RuntimeError> {
        if let LiteralValue::NumberLiteral(n) = value {
            Ok(*n)
        } else {
            Err(RuntimeError {
                message: "Operand must be a number.".to_string(),
                line,
            })
        }
    }

    // Helper method to determine if a value is "truthy"
    fn is_truthy(&self, value: &LiteralValue) -> bool {
        match value {
            LiteralValue::Nil => false,
            LiteralValue::BooleanLiteral(b) => *b,
            _ => true, // All other values are truthy
        }
    }
    
}
