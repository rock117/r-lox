use crate::environment::Environment;
use crate::error::{LoxError, ParseError};
use crate::interpreter::Interpreter;
use crate::object::Object;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub enum NativeFunction {
    Clock(Clock),
}
#[derive(Debug, Clone)]
struct Clock;

impl NativeFunction {
    pub fn clock() -> Self {
        NativeFunction::Clock(Clock)
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Option<Object>>,
    ) -> Result<Option<Object>, LoxError> {
        match self {
            NativeFunction::Clock(clock) => clock.call(interpreter, arguments),
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            NativeFunction::Clock(clock) => clock.arity(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            NativeFunction::Clock(clock) => clock.to_string(),
        }
    }
}

impl Clock {
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Option<Object>>,
    ) -> Result<Option<Object>, LoxError> {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .map(|v| v.as_secs())
            .unwrap(); // TODO
        Ok(Some(Object::Number(since_the_epoch as f64)))
    }

    pub fn arity(&self) -> usize {
        0
    }

    pub fn to_string(&self) -> String {
        "<native fn clock>".into()
    }
}
