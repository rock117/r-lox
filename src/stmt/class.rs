use crate::stmt::function::Function;
use crate::token::Token;

#[derive(Debug, Clone)]
pub(crate) struct Class {
    pub name: Token,
    pub methods: Vec<Function>,
}
