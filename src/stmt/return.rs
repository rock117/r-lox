use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, Clone)]
pub(crate) struct Return {
    pub keyword: Token,
    pub value: Option<Expr>,
}