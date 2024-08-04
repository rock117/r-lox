use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone)]
pub(crate) struct Var {
    pub name: Token,
    pub initializer: Option<Expr>,
}
