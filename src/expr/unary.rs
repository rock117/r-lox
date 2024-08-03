use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone)]
pub(crate) struct Unary {
    pub(crate) operator: Token,
    pub(crate) right: Expr,
}

impl Unary {
    pub fn new(operator: Token, right: Expr) -> Self {
        Self { operator, right }
    }
}
