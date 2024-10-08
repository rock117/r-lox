use crate::expr::{Expr, Visitor};
#[derive(Debug, Clone)]
pub(crate) struct Grouping {
    pub(crate) expression: Expr,
}

impl Grouping {
    pub fn new(expression: Expr) -> Self {
        Self { expression }
    }
}
