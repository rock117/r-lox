use crate::token::token_type::TokenType;
use crate::token::Literal::{Number, Str};
use std::fmt::{Display, Formatter, Write};

pub(crate) mod token_type;

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    Str(String),
}
impl Literal {
    pub fn string(str: String) -> Self {
        Str(str)
    }
    pub fn number(number: f64) -> Self {
        Number(number)
    }
}
impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number(n) => write!(f, "{}", n.to_string()),
            Str(str) => write!(f, "{}", str.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) r#type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<Literal>,
    pub(crate) line: usize,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
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
