use std::fmt::Display;
use crate::expr::{Expr, Visitor};
use crate::expr::binary::Binary;
use crate::expr::grouping::Grouping;
use crate::expr::literal::Literal;
use crate::expr::unary::Unary;

pub(crate) struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }
    pub fn interpret<E: Expr>(expression: &E) {
        todo!()
    }
    fn evaluate<E: Expr>(&self, expr: &E) {
        return expr.accept(self);
    }
}

impl<T> Visitor<T> for Interpreter {
    fn visit_binary_expr<L: Expr, R: Expr>(&self, expr: &Binary<L, R>) -> T {
        todo!()
    }

    fn visit_grouping_expr<E: Expr>(&self, expr: &Grouping<E>) -> T {
        todo!()
    }

    fn visit_literal_expr<E: Display>(&self, expr: &Literal<E>) -> T {
        todo!()
    }

    fn visit_unary_expr<E: Expr>(&self, expr: &Unary<E>) -> T {
        todo!()
    }
}