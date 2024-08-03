use crate::object::Object::{Boolean, Number, Str};
use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Object {
    Str(String),
    Number(f64),
    Boolean(bool),
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

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Str(v) => write!(f, "{}", v),
            Object::Number(v) => write!(f, "{}", v),
            Object::Boolean(v) => write!(f, "{}", v),
        }
    }
}
