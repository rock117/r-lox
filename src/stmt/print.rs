use crate::expr::Expr;
#[derive(Clone)]
pub(crate) struct Print {
    pub(crate) expression: Expr,
}
