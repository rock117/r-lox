use crate::environment::Environment;
use crate::error::LoxError;
use crate::function::lox_function::LoxFunction;
use crate::instance::LoxInstance;
use crate::interpreter::Interpreter;
use crate::object::Object;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub(crate) struct LoxClass {
    pub name: String,
    methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(name: String, methods: HashMap<String, LoxFunction>) -> Self {
        LoxClass { name, methods }
    }

    pub(crate) fn find_method(&self, name: &str) -> Option<LoxFunction> {
        self.methods.get(name).map(|v| v.clone())
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

impl PartialEq for LoxClass {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}
