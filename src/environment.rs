use crate::error::{ParseError, VarNotDefinedError};
use crate::object::Object;
use crate::token::Token;
use std::collections::HashMap;

pub(crate) struct Environment {
    values: HashMap<String, Option<Object>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }
    pub fn get(&self, name: &Token) -> Result<&Option<Object>, ParseError> {
        let value = self.values.get(&name.lexeme);
        match value {
            None => Err(ParseError::new(
                name.clone(),
                format!("Undefined variable '{}'.", name.lexeme),
            )), // "Undefined variable '" + name.lexeme + "'."
            Some(v) => Ok(v),
        }
    }

    pub fn define(&mut self, name: String, value: Option<Object>) {
        self.values.insert(name, value);
    }
}
