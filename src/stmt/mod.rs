pub(crate) mod block;
pub(crate) mod expression;
pub(crate) mod print;
pub(crate) mod var;

use crate::error::ParseError;
use crate::expr::Expr;
use crate::expr::Expr::{Binary, Grouping, Literal, Unary};
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::stmt;
use crate::token::Token;

#[derive(Clone)]
pub(crate) enum Stmt {
    Expression(expression::Expression),
    Print(print::Print),
    Var(var::Var),
    Block(block::Block),
}

impl Stmt {
    pub fn accept(&self, visitor: &mut Interpreter) -> Result<Option<Object>, ParseError> {
        match self {
            Stmt::Expression(v) => visitor
                .visit_expression_stmt(v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::Print(v) => visitor
                .visit_print_stmt(v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::Var(v) => visitor
                .visit_var_stmt(v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::Block(v) => visitor
                .visit_block_stmt(v.clone())
                .map(|_| Some(Object::Void)),
        }
    }

    pub fn print(expression: Expr) -> Self {
        Stmt::Print(print::Print { expression })
    }
    pub fn expression(expression: Expr) -> Self {
        Stmt::Expression(expression::Expression { expression })
    }

    pub fn var(token: Token, initializer: Option<Expr>) -> Self {
        Stmt::Var(var::Var {
            name: token,
            initializer,
        })
    }
    pub fn block(statements: Vec<Stmt>) -> Self {
        Stmt::Block(block::Block { statements })
    }
}

pub(crate) trait Visitor {
    /// execute expression, ignore result
    fn visit_expression_stmt(&mut self, stmt: expression::Expression) -> Result<(), ParseError>;

    /// print statement
    fn visit_print_stmt(&mut self, stmt: print::Print) -> Result<(), ParseError>;

    /// define var
    fn visit_var_stmt(&mut self, stmt: var::Var) -> Result<(), ParseError>;

    /// execute block
    fn visit_block_stmt(&mut self, stmt: block::Block) -> Result<(), ParseError>;
}
