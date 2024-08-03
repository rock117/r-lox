use crate::object::Object;
use crate::token::token_type::TokenType;
use std::fmt::{Display, Formatter, Write};

pub(crate) mod token_type;

#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) r#type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<Object>,
    pub(crate) line: usize,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Self {
        Self {
            r#type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.r#type,
            self.lexeme,
            self.literal
                .clone()
                .map(|v| v.to_string())
                .unwrap_or("".into())
        )
    }
}
