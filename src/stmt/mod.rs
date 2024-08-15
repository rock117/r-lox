pub(crate) mod expression;
pub(crate) mod print;
pub(crate) mod var;

use crate::error::ParseError;
use crate::expr::Expr;
use crate::expr::Expr::{Binary, Grouping, Literal, Unary};
use crate::object::Object;
use crate::stmt;
use crate::token::Token;

pub(crate) enum Stmt {
    Expression(expression::Expression),
    Print(print::Print),
    Var(var::Var),
}

impl Stmt {
    pub fn accept<V: Visitor>(&self, visitor: &V) -> Result<Option<Object>, ParseError> {
        match self {
            Stmt::Expression(v) => visitor
                .visit_expression_stmt(v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::Print(v) => visitor
                .visit_print_stmt(v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::Var(_) => {}
        }
    }

    pub fn print(expression: Expr) -> Self {
        Stmt::Print(print::Print { expression })
    }
    pub fn expression(expression: Expr) -> Self {
        Stmt::Expression(expression::Expression { expression })
    }

    pub fn var(token: Token, initializer: Option<Expr>) -> Self {
        Stmt::Var(var::Var { name, initializer })
    }
}

pub(crate) trait Visitor {

    /// evalue expression, ignore result
    fn visit_expression_stmt(&self, stmt: expression::Expression) -> Result<(), ParseError>;

    /// print statement
    fn visit_print_stmt(&self, stmt: print::Print) -> Result<(), ParseError>;

    /// define var
    fn visit_var_stmt(&self, stmt: var::Var) -> Result<(), ParseError>;
}
