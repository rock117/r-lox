use crate::expr::{Expr, Visitor};
use crate::token::Token;

pub(crate) struct Unary<E: Expr> {
    pub(crate) operator: Token,
    pub(crate) right: E,
}
impl<E: Expr> Unary<E> {
    pub fn new(operator: Token, right: E) -> Self {
        Self { operator, right }
    }
}

impl<E: Expr> Expr for Unary<E> {
    fn accept<T>(&self, visitor: &impl Visitor<T>) -> T {
        visitor.visit_unary_expr(self)
    }
}
