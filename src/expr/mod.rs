use std::fmt::Display;
use crate::expr::binary::Binary;
use crate::expr::grouping::Grouping;
use crate::expr::literal::Literal;
use crate::expr::unary::Unary;

mod ast_printer;
mod binary;
mod grouping;
mod literal;
mod unary;

trait Expr {
    fn accept<T>(&self, visitor: &impl Visitor<T>) -> T;
}

pub(crate) trait Visitor<T> {
    fn visit_binary_expr<L: Expr, R: Expr>(&self, expr: &Binary<L, R>) -> T;

    fn visit_grouping_expr<E: Expr>(&self, expr: &Grouping<E>) -> T;

    fn visit_literal_expr<E: Display>(&self, expr: &Literal<E>) -> T;

    fn visit_unary_expr<E: Expr>(&self, expr: &Unary<E>) -> T;
}
