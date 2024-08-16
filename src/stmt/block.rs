use crate::stmt::Stmt;

#[derive(Clone)]
pub(crate) struct Block {
    pub(crate) statements: Vec<Stmt>,
}
