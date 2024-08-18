use crate::expr::Expr;
#[derive(Debug, Clone)]
pub(crate) struct Print {
    pub(crate) expression: Expr,
}
