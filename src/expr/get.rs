use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, Clone)]
pub(crate) struct Get {
    pub object: Expr,
    pub name: Token,
}
