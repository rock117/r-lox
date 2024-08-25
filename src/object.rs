use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use crate::class::LoxClass;
use crate::instance::LoxInstance;

use crate::object::Object::{Boolean, Number, Str};

#[derive(Clone, PartialEq)]
pub(crate) enum Object {
    Str(String),
    Number(f64),
    Boolean(bool),
    Void,
    Function(Box<crate::function::LoxCallable>),
    Class(LoxClass),
    Instance(LoxInstance),
}

impl Object {
    pub fn string(str: String) -> Self {
        Str(str)
    }
    pub fn number(n: f64) -> Self {
        Number(n)
    }
    pub fn boolean(b: bool) -> Self {
        Boolean(b)
    }

    pub fn is_equal(&self, other: &Self) -> bool {
        match (self, other) {
            (Str(a), Str(b)) => a == b,
            (Boolean(a), Boolean(b)) => a == b,
            (Number(a), Number(b)) => a.partial_cmp(b).unwrap_or(Ordering::Less) == Ordering::Equal,
            _ => false,
        }
    }
}

fn to_string(object: &Object) -> String {
    match object {
        Str(v) => format!("{}", v),
        Number(v) => format!("{}", v),
        Boolean(v) => format!("{}", v),
        Object::Void => "".into(),
        Object::Function(f) => f.to_string(),
        Object::Class(class) => class.to_string(),
        Object::Instance(instance) => instance.to_string()
    }
}
impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_string(self))
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_string(self))
    }
}
