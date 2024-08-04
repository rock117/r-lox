pub(crate) mod expression;
pub(crate) mod print;
mod var;

use crate::error::ParseError;
use crate::expr::Expr;
use crate::expr::Expr::{Binary, Grouping, Literal, Unary};
use crate::object::Object;
use crate::stmt;

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
        }
    }

    pub fn print(expression: Expr) -> Self {
        Stmt::Print(print::Print { expression })
    }
    pub fn expression(expression: Expr) -> Self {
        Stmt::Expression(expression::Expression { expression })
    }
}

pub(crate) trait Visitor {
    fn visit_expression_stmt(&self, stmt: expression::Expression) -> Result<(), ParseError>;

    fn visit_print_stmt(&self, stmt: print::Print) -> Result<(), ParseError>;
}
