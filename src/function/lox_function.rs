use crate::environment::Environment;
use crate::error::ParseError;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::stmt;
#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub(crate) declaration: stmt::function::Function,
}

impl LoxFunction {
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Option<Object>>,
    ) -> Result<Option<Object>, ParseError> {
        let mut environment = Environment::new(); // TODO (interpreter.globals);
        for i in 0..self.declaration.params.len() {
            if let (Some(param), Some(arg)) = (self.declaration.params.get(i), arguments.get(i)) {
                environment.define(param.lexeme.clone(), arg.clone());
            }
        }
        let result = interpreter.execute_block(self.declaration.body.clone(), environment);
        if let Err(returnValue) = result {
           // return returnValue.value
            todo!()
        }
        Ok(None)
    }

    pub fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    pub fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme)
    }
}
