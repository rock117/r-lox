use std::fmt::{Debug, Display};

use crate::error::LoxError;
use crate::expr::Expr::{Assign, Binary, Call, Grouping, Literal, Logical, Unary, Variable};
use crate::object::Object;
use crate::token::Token;

pub mod assign;
pub mod ast_printer;
pub mod binary;
pub mod call;
pub mod grouping;
pub mod literal;
pub mod logical;
pub mod unary;
pub(crate) mod variable;

#[derive(Debug, Clone)]
pub enum Expr {
    Assign(Box<assign::Assign>),
    Binary(Box<binary::Binary>),
    Grouping(Box<grouping::Grouping>),
    Logical(Box<logical::Logical>),
    Literal(Box<literal::Literal>),
    Unary(Box<unary::Unary>),
    Variable(variable::Variable),
    Call(Box<call::Call>),
}

impl Expr {
    pub fn assign(name: Token, expr: Expr) -> Self {
        Assign(Box::new(assign::Assign { name, value: expr }))
    }
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
    pub fn variable(name: Token) -> Self {
        Variable(variable::Variable { name })
    }
    pub fn logical(left: Expr, operator: Token, right: Expr) -> Self {
        Logical(Box::new(logical::Logical {
            left,
            operator,
            right,
        }))
    }

    pub fn call(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Self {
        Call(Box::new(call::Call {
            callee,
            paren,
            arguments,
        }))
    }

    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<Option<Object>, LoxError> {
        match self {
            Binary(v) => visitor.visit_binary_expr((**v).clone()),
            Grouping(v) => visitor.visit_grouping_expr((**v).clone()),
            Literal(v) => visitor.visit_literal_expr((**v).clone()),
            Unary(v) => visitor.visit_unary_expr((**v).clone()),
            Assign(v) => visitor.visit_assign_expr(*v.clone()),
            Variable(v) => visitor.visit_variable_expr(v.clone()),
            Logical(v) => visitor.visit_logical_expr(*v.clone()),
            Call(v) => visitor.visit_call_expr(*v.clone()),
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

    fn visit_literal_expr(&self, expr: literal::Literal) -> Result<Option<Object>, LoxError>;

    fn visit_grouping_expr(&mut self, expr: grouping::Grouping)
        -> Result<Option<Object>, LoxError>;

    fn visit_unary_expr(&mut self, expr: unary::Unary) -> Result<Option<Object>, LoxError>;

    fn visit_binary_expr(&mut self, expr: binary::Binary) -> Result<Option<Object>, LoxError>;

    /// read expr value
    fn visit_variable_expr(&self, expr: variable::Variable) -> Result<Option<Object>, LoxError>;

    /// evalue right value and assign to left var name
    fn visit_assign_expr(&mut self, expr: assign::Assign) -> Result<Option<Object>, LoxError>;

    /// evalue logical expression
    fn visit_logical_expr(&mut self, expr: logical::Logical) -> Result<Option<Object>, LoxError>;

    /// execute function
    fn visit_call_expr(&mut self, expr: call::Call) -> Result<Option<Object>, LoxError>;
}
