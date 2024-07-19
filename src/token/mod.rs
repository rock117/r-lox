use crate::token::token_type::TokenType;
use std::fmt::{Display, Formatter, Write};

pub(crate) mod token_type;

#[derive(Debug, Clone)]
pub struct Token {
    r#type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
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
            self.literal.clone().unwrap_or_default()
        )
    }
}
