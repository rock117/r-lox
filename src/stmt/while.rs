use crate::expr::Expr;
use crate::stmt::Stmt;

#[derive(Debug, Clone)]
pub(crate) struct While {
    pub(crate) condition: Expr,
    pub(crate) body: Stmt,
}
