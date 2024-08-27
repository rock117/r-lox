use crate::expr::variable::Variable;
use crate::stmt::function::Function;
use crate::token::Token;

#[derive(Debug, Clone)]
pub(crate) struct Class {
    pub name: Token,
    pub superclass: Option<Variable>,
    pub methods: Vec<Function>,
}
