use crate::expr::Expr;
use crate::stmt::Stmt;

#[derive(Clone)]
pub(crate) struct If {
    pub(crate) condition: Expr,
    pub(crate) thenBranch: Stmt,
    pub(crate) elseBranch: Option<Stmt>,
}
