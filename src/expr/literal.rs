use crate::object::Object;
use std::fmt::Debug;

/// /// an expression which is a literal
#[derive(Debug, Clone)]
pub(crate) struct Literal {
    pub(crate) value: Option<Object>,
}

impl Literal {
    pub fn new(value: Option<Object>) -> Self {
        Self { value }
    }
}
