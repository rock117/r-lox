use crate::environment::Environment;
use crate::error::LoxError;
use crate::expr::variable::Variable;
use crate::function::lox_function::LoxFunction;
use crate::instance::LoxInstance;
use crate::interpreter::Interpreter;
use crate::object::Object;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub(crate) struct LoxClass {
    pub name: String,
    superclass: Option<Box<LoxClass>>,
    methods: HashMap<String, LoxFunction>,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum ClassType {
    NONE,
    CLASS,
    SUBCLASS,
}

impl LoxClass {
    pub fn new(
        name: String,
        superclass: Option<LoxClass>,
        methods: HashMap<String, LoxFunction>,
    ) -> Self {
        LoxClass {
            name,
            superclass: superclass.map(|c| Box::new(c)),
            methods,
        }
    }

    pub(crate) fn find_method(&self, name: &str) -> Option<LoxFunction> {
        if let Some(superclass) = &self.superclass {
            return superclass.find_method(name);
        }
        self.methods.get(name).map(|v| v.clone())
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Option<Object>>,
    ) -> Result<Option<Object>, LoxError> {
        let instance = LoxInstance::new(self.clone());
        let initializer = self.find_method("init");
        if let Some(initializer) = initializer {
            initializer
                .bind(instance.clone())
                .call(interpreter, arguments)?;
        }
        Ok(Some(Object::Instance(instance)))
    }

    pub fn arity(&self) -> usize {
        let initializer = self.find_method("init");
        match initializer {
            None => 0,
            Some(initializer) => initializer.arity(),
        }
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
