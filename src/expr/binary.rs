use crate::expr::{Expr, Visitor};
use crate::token::Token;

pub(crate) struct Binary<L: Expr, R: Expr> {
    pub(crate) left: L,
    pub(crate) operator: Token,
    pub(crate) right: R,
}
impl<L: Expr, R: Expr> Binary<L, R> {
    pub fn new(left: L, operator: Token, right: R) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl<L: Expr, R: Expr> Expr for Binary<L, R> {
    fn accept<T>(&self, visitor: &impl Visitor<T>) -> T {
        visitor.visit_binary_expr(self)
    }
}
