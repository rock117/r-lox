use crate::error::ParseError;
use crate::expr::Expr::{Binary, Grouping, Literal, Unary};
use crate::object::Object;
use crate::token::Token;
use std::fmt::{Debug, Display};

pub mod ast_printer;
pub mod binary;
pub mod grouping;
pub mod literal;
pub mod unary;
mod variable;

#[derive(Clone)]
pub enum Expr {
    Binary(Box<binary::Binary>),
    Grouping(Box<grouping::Grouping>),
    Literal(Box<literal::Literal>),
    Unary(Box<unary::Unary>),
    Variable(variable::Variable),
}

impl Expr {
    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Binary(Box::new(binary::Binary::new(left, operator, right)))
    }
    pub fn grouping(expression: Expr) -> Self {
        Grouping(Box::new(grouping::Grouping::new(expression)))
    }
    pub fn literal(object: Option<Object>) -> Self {
        Literal(Box::new(literal::Literal::new(object)))
    }
    pub fn unary(operator: Token, right: Expr) -> Self {
        Unary(Box::new(unary::Unary::new(operator, right)))
    }
    pub fn accept<V: Visitor>(&self, visitor: &V) -> Result<Option<Object>, ParseError> {
        match self {
            Binary(v) => visitor.visit_binary_expr((**v).clone()),
            Grouping(v) => visitor.visit_grouping_expr((**v).clone()),
            Literal(v) => visitor.visit_literal_expr((**v).clone()),
            Unary(v) => visitor.visit_unary_expr((**v).clone()),
        }
    }
}

pub(crate) trait Visitor {
    // R visitAssignExpr(Assign expr);
    // R visitBinaryExpr(Binary expr);
    // R visitCallExpr(Call expr);
    // R visitGetExpr(Get expr);
    // R visitGroupingExpr(Grouping expr);
    // R visitLiteralExpr(Literal expr);
    // R visitLogicalExpr(Logical expr);
    // R visitSetExpr(Set expr);
    // R visitSuperExpr(Super expr);
    // R visitThisExpr(This expr);
    // R visitUnaryExpr(Unary expr);
    // R visitVariableExpr(Variable expr);

    fn visit_literal_expr(&self, expr: literal::Literal) -> Result<Option<Object>, ParseError>;

    fn visit_grouping_expr(&self, expr: grouping::Grouping) -> Result<Option<Object>, ParseError>;

    fn visit_unary_expr(&self, expr: unary::Unary) -> Result<Option<Object>, ParseError>;

    fn visit_binary_expr(&self, expr: binary::Binary) -> Result<Option<Object>, ParseError>;
}
