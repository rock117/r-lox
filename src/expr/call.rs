use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, Clone)]
pub(crate) struct Call {
    /// function name
    pub callee: Expr,
    /// right paren, use when a runtime error occur caused by a function call,  report function’s location
    pub paren: Token,
    pub arguments: Vec<Expr>,
}
