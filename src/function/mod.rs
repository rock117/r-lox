pub mod lox_function;
pub mod native_function;

use crate::class::LoxClass;
use crate::error::{LoxError, ParseError};
use crate::interpreter::Interpreter;
use crate::object::Object;

#[derive(Debug, Clone)]
pub enum LoxCallable {
    LoxFunction(lox_function::LoxFunction),
    NativeFunction(native_function::NativeFunction),
    LoxClass(LoxClass),
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FunctionType {
    NONE,
    FUNCTION,
    METHOD,
    INITIALIZER
}

impl LoxCallable {
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Option<Object>>,
    ) -> Result<Option<Object>, LoxError> {
        match self {
            LoxCallable::LoxFunction(f) => f.call(interpreter, arguments),
            LoxCallable::NativeFunction(f) => f.call(interpreter, arguments),
            LoxCallable::LoxClass(class) => class.call(interpreter, arguments),
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            LoxCallable::LoxFunction(f) => f.arity(),
            LoxCallable::NativeFunction(f) => f.arity(),
            LoxCallable::LoxClass(class) => class.arity(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            LoxCallable::LoxFunction(f) => f.to_string(),
            LoxCallable::NativeFunction(f) => f.to_string(),
            LoxCallable::LoxClass(class) => class.to_string(),
        }
    }
}

impl PartialEq for LoxCallable {
    fn eq(&self, other: &Self) -> bool {
        false // not matter whether true or false
    }
}
