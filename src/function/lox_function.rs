use crate::environment::Environment;
use crate::error::LoxError;
use crate::instance::LoxInstance;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::stmt;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub(crate) declaration: stmt::function::Function,
    pub(crate) closure: Rc<RefCell<Environment>>,
    pub(crate) is_initializer: bool
}

impl LoxFunction {
    pub(crate) fn bind(&self, instance: LoxInstance) -> LoxFunction {
        let mut environment = Environment::new_from_enclosing(self.closure.clone());
        environment.define("this".into(), Some(Object::Instance(instance)));
        LoxFunction {
            is_initializer: self.is_initializer,
            declaration: self.declaration.clone(),
            closure: Rc::new(RefCell::new(environment)),
        }
    }

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
            Ok(_) => {
                if self.is_initializer {
                    match self.closure.borrow().get_at(0, "this") {
                        Some(v) => return Ok(v),
                        None => {}
                    }
                }
                Ok(None)
            },
            Err(LoxError::ReturnError(returnValue)) => {
                if self.is_initializer {
                    if let Some(fun) = self.closure.borrow().get_at(0, "this".into()) {
                        return Ok(fun)
                    }
                }
                Ok(returnValue.value)
            },
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
