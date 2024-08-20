use crate::environment::Environment;
use crate::error::LoxError;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::stmt;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub(crate) declaration: stmt::function::Function,
    pub(crate) closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Option<Object>>,
    ) -> Result<Option<Object>, LoxError> {
        let mut environment = Environment::new_from_enclosing(self.closure.clone());
        for i in 0..self.declaration.params.len() {
            if let (Some(param), Some(arg)) = (self.declaration.params.get(i), arguments.get(i)) {
                environment.define(param.lexeme.clone(), arg.clone());
            }
        }
        let result = interpreter.execute_block(self.declaration.body.clone(), environment);
        match result {
            Ok(_) => Ok(None),
            Err(LoxError::ReturnError(returnValue)) => Ok(returnValue.value),
            Err(e) => Err(e),
        }
    }

    pub fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    pub fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme)
    }
}
