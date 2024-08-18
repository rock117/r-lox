use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, Clone)]
pub(crate) struct Assign {
    pub name: Token,
    pub value: Expr,
}
