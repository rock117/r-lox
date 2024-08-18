use crate::expr::Expr;
use crate::token::Token;
#[derive(Debug, Clone)]
pub(crate) struct Logical {
    pub(crate) left: Expr,
    pub(crate) operator: Token,
    pub(crate) right: Expr,
}
