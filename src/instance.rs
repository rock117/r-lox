use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::class::LoxClass;
use crate::error::LoxError;
use crate::object::Object;
use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LoxInstance {
    pub klass: LoxClass,
    fields: HashMap<String, Object>,
}

impl LoxInstance {
    pub fn new(klass: LoxClass) -> Self {
        LoxInstance {klass, fields: HashMap::new()}
    }

    pub fn get(&self, name: Token ) -> Result<Object, LoxError> {
        if let Some(obj) = self.fields.get(&name.lexeme) {
            return Ok(obj.clone());
        }
        Err(LoxError::new_parse_error(name.clone(), format!("Undefined property '{}'.", name.lexeme)))
    }
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.klass.name)
    }
}