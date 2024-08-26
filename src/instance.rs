use crate::class::LoxClass;
use crate::error::LoxError;
use crate::function::LoxCallable;
use crate::function::LoxCallable::LoxFunction;
use crate::object::Object;
use crate::token::Token;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LoxInstance {
    pub klass: LoxClass,
    fields: HashMap<String, Object>,
}

impl LoxInstance {
    pub fn new(klass: LoxClass) -> Self {
        LoxInstance {
            klass,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: Token) -> Result<Object, LoxError> {
        if let Some(obj) = self.fields.get(&name.lexeme) {
            return Ok(obj.clone());
        }
        let method = self.klass.find_method(&name.lexeme);
        if let Some(method) = method {
            return Ok(Object::Function(Box::new(LoxFunction(
                method.bind(self.clone()),
            ))));
        }
        Err(LoxError::new_parse_error(
            name.clone(),
            format!("Undefined property '{}'.", name.lexeme),
        ))
    }

    pub fn set(&mut self, name: &Token, value: Object) -> Result<Object, LoxError> {
        self.fields.insert(name.lexeme.clone(), value);
        Ok(Object::Void)
    }
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.klass.name)
    }
}
