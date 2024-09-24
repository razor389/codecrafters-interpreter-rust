use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone)]
pub enum Stmt {
    Expression(Expr),                   // An expression statement
    Print(Expr),                        // A print statement
    Var { name: Token, initializer: Option<Expr> },  // Variable declaration
    Block(Vec<Stmt>),
}
