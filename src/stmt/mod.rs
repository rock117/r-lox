use crate::expr::Expr;

pub(crate) enum Stmt {
    Expression { expression: Expr },
    Stmt { expression: Expr },
}

impl Stmt {
    pub fn accept<V: Visitor>(visitor: &V) -> () {}
}

pub(crate) trait Visitor {
    fn visit_block_stmt(stmt: &str) -> (); //
}
