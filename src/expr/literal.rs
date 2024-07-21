use crate::expr::{Expr, Visitor};
use std::fmt::Display;

pub(crate) struct Literal<T: Display> {
    pub(crate) value: Option<T>,
}
impl<T: Display> Literal<T> {
    pub fn new(value: Option<T>) -> Self {
        Self { value }
    }
}

impl<T: Display> Expr for Literal<T> {
    fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
        visitor.visit_literal_expr(self)
    }
}
