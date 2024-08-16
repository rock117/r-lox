use crate::error::ParseError;
use crate::object::Object;
use crate::token::Token;
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Option<Object>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_from_enclosing(enclosing: Environment) -> Self {
        Environment {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<&Option<Object>, ParseError> {
        let value = self.values.get(&name.lexeme);
        match value {
            Some(v) => Ok(v),
            None => {
                if let Some(enclosing) = &self.enclosing {
                    return enclosing.get(name);
                }

                Err(ParseError::new(
                    name.clone(),
                    format!("Undefined variable '{}'.", name.lexeme),
                ))
            }
        }
    }

    pub fn define(&mut self, name: String, value: Option<Object>) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &Token, value: Option<Object>) -> Result<(), ParseError> {
        if (self.values.contains_key(&name.lexeme)) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        if let Some(enclosing) = &mut self.enclosing {
            return enclosing.assign(name, value);
        }

        return Err(ParseError::new(
            name.clone(),
            format!("Undefined variable '{}'.", name.lexeme),
        ));
    }
}
