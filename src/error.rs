use crate::object::Object;
use crate::token::Token;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoxError {
    ParseError(ParseError),
    ReturnError(Return),
}

#[derive(Debug, Error)]
pub(crate) struct ParseError {
    pub token: Token,
    pub message: String,
}

#[derive(Debug, Error)]
pub struct Return {
    pub(crate) value: Option<Object>,
}

impl LoxError {
    pub fn new_parse_error(token: Token, message: String) -> Self {
        LoxError::ParseError(ParseError { token, message })
    }
}

impl ParseError {
    pub fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "message: {}, token: {:?}", self.message, self.token)
    }
}

impl Display for Return {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "return {:?}", self.value)
    }
}

impl Display for LoxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxError::ParseError(e) => e.fmt(f),
            LoxError::ReturnError(e) => e.fmt(f),
        }
    }
}
