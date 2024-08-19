use crate::stmt::Stmt;
use crate::token::Token;

#[derive(Debug, Clone)]
pub(crate) struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}
