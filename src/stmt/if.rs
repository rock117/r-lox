use crate::expr::Expr;
use crate::stmt::Stmt;

#[derive(Debug, Clone)]
pub(crate) struct If {
    pub(crate) condition: Expr,
    pub(crate) then_branch: Stmt,
    pub(crate) else_branch: Option<Stmt>,
}
