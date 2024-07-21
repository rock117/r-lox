use crate::expr::{Expr, Visitor};

pub(crate) struct Grouping<E: Expr> {
    pub(crate) expression: E,
}

impl<E: Expr> Grouping<E> {
    pub fn new(expression: E) -> Self {
        Self { expression }
    }
}

impl<E: Expr> Expr for Grouping<E> {
    fn accept<T>(&self, visitor: &impl Visitor<T>) -> T {
        visitor.visit_grouping_expr(self)
    }
}
