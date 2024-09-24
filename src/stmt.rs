use crate::expr::Expr;
use crate::token::Token;

pub enum Stmt {
    Expression(Expr),                   // An expression statement
    Print(Expr),                        // A print statement
    Var { name: Token, initializer: Option<Expr> },  // Variable declaration
}
