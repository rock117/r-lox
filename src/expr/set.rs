use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone, Debug)]
pub(crate) struct Set {
    pub object: Expr,
    pub name: Token,
    pub value: Expr,
}
