use crate::error::ParseError;
use crate::object::Object;
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct Environment {
    /// parent scope's env
    enclosing: Option<Rc<RefCell<Environment>>>,
    /// current scope's env
    values: HashMap<String, Option<Object>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_from_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Option<Object>, ParseError> {
        let value = self.values.get(&name.lexeme);
        match value {
            Some(v) => Ok(v.clone()),
            None => {
                if let Some(enclosing) = self.enclosing.clone() {
                    return Ok(enclosing.borrow_mut().get(name)?.clone());
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
            return enclosing.borrow_mut().assign(name, value);
        }

        return Err(ParseError::new(
            name.clone(),
            format!("Undefined variable '{}'.", name.lexeme),
        ));
    }
}
