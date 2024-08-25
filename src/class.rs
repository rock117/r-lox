use std::fmt::{Display, Formatter};
use crate::environment::Environment;
use crate::error::LoxError;
use crate::instance::LoxInstance;
use crate::interpreter::Interpreter;
use crate::object::Object;

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct LoxClass {
    pub name: String
}

impl LoxClass {
    pub fn new(name: String) -> Self {
        LoxClass { name }
    }
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Option<Object>>,
    ) -> Result<Option<Object>, LoxError> {
        Ok(Some(Object::Instance(LoxInstance::new(self.clone()))))
    }

    pub fn arity(&self) -> usize {
        0
    }
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}