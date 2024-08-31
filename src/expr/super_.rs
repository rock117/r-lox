use crate::token::Token;
#[derive(Clone, Debug)]
pub(crate) struct Super {
    pub keyword: Token,
    pub method: Token,
}
