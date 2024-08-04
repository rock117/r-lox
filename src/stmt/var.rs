use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone)]
pub(crate) struct Var {
    name: Token,
    initializer: Option<Expr>,
}
