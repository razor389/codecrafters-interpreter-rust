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
#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, LiteralValue>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

     // Create a new environment that has a parent (enclosing scope)
     pub fn from_enclosing(enclosing: Environment) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str, line: usize) -> Result<LiteralValue, RuntimeError> {
        log::debug!("getting var: {}", name);
        if let Some(value) = self.values.get(name) {
            log::debug!("got {:?}", value.clone());
            Ok(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            // If not found in the current environment, check the enclosing one
            log::debug!("not found in current, checking enclosing");
            enclosing.get(name, line)
        } else {
            Err(RuntimeError {
                message: format!("Undefined variable '{}'.", name),
                line,
            })
        }
    }

    pub fn assign(&mut self, name: &str, value: LiteralValue, line: usize) -> Result<(), RuntimeError> {
        if self.values.contains_key(name) {
            log::debug!("assigning {:?} to {}", value, name);
            self.values.insert(name.to_string(), value);
            Ok(())
        } else if let Some(enclosing) = &mut self.enclosing {
            log::debug!("var {} not declared in current scope, trying to assign in enclosing", name);
            // If not found in the current environment, try to assign in the enclosing one
            enclosing.assign(name, value, line)
        } else {
            Err(RuntimeError {
                message: format!("Undefined variable '{}'.", name),
                line,
            })
        }
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

     // Execute a block of statements in a new environment
    fn execute_block(&mut self, statements: &[Stmt], environment: Environment) -> Result<(), RuntimeError> {
        log::debug!("--- ENTERING BLOCK ---");
        log::debug!("Environment before block: {:?}", self.environment.values);

        // Push a new environment for the block scope, while keeping the current environment accessible.
        let new_environment = Environment::from_enclosing(self.environment.clone());
        let mut previous = std::mem::replace(&mut self.environment, new_environment);

        log::debug!("New environment inside block: {:?}", self.environment.values);

        // Execute the block
        let result = self.interpret(statements.to_vec());

        // Instead of fully restoring the previous environment, merge changes back to the enclosing scope
        for (key, value) in self.environment.values.iter() {
            log::debug!("Merging variable {} with value {:?}", key, value);
            previous.define(key.clone(), value.clone());
        }

        // Restore the environment (but merged changes persist)
        self.environment = previous;

        log::debug!("Restored environment after block: {:?}", self.environment.values);
        log::debug!("--- EXITING BLOCK ---");
        result
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
                self.environment.define(name.lexeme.clone(), value.clone());
                log::debug!("defined variable {} with value: {:?}", name.lexeme.clone(), value);
                Ok(())
            }
            Stmt::Block(statements) => {
                // Create a new environment and execute the block
                self.execute_block(statements, Environment::from_enclosing(self.environment.clone()))
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
    pub fn evaluate(&mut self, expr: &Expr) -> Result<LiteralValue, RuntimeError> {
        match expr {
            Expr::Literal(value) => self.visit_literal(value),
            Expr::Assign { name, value } => {
                let new_value = self.evaluate(value)?;
                self.environment.assign(&name.lexeme, new_value.clone(), name.line)?;
                Ok(new_value)
            },
            Expr::Variable(name) => self.environment.get(&name.lexeme, name.line),
            Expr::Unary { operator, right } => self.visit_unary(operator, right),
            Expr::Binary { left, operator, right } => self.visit_binary(left, operator, right),
            Expr::Grouping(expr) => self.visit_grouping(expr),
        }
    }
    

    fn visit_literal(&self, value: &LiteralValue) -> Result<LiteralValue, RuntimeError> {
        Ok(value.clone()) // Return the literal value as-is
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Result<LiteralValue, RuntimeError> {
        self.evaluate(expr)
    }
    

    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Result<LiteralValue, RuntimeError> {
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
    

    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<LiteralValue, RuntimeError> {
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
