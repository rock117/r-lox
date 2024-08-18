use crate::stmt::Stmt;

#[derive(Debug, Clone)]
pub(crate) struct Block {
    pub(crate) statements: Vec<Stmt>,
}
