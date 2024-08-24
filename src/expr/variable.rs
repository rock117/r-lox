use crate::token::Token;
/// an expression which is a variable
#[derive(Debug, Clone)]
pub(crate) struct Variable {
    pub name: Token,
}
