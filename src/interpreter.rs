use crate::error::ParseError;
use crate::expr::binary::Binary;
use crate::expr::grouping::Grouping;
use crate::expr::literal::Literal;
use crate::expr::unary::Unary;
use crate::expr::{Expr, Visitor};
use crate::lox::Lox;
use crate::object::Object;
use crate::token::token_type::TokenType;
use std::fmt::Display;

pub(crate) struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }
    pub fn interpret(&self, expression: &Expr) {
        let v = self.evaluate(&expression);
        match v {
            Ok(v) => println!("{}", self.stringify(v)),
            Err(e) => Lox::runtime_error(e), // Lox::runtimeError(&e)
        }
    }

    fn evaluate(&self, expr: &Expr) -> Result<Option<Object>, ParseError> {
        return expr.accept(self);
    }

    fn stringify(&self, object: Option<Object>) -> String {
        let Some(object) = object else {
            return "null".into();
        };
        match object {
            Object::Str(v) => v,
            Object::Number(v) => {
                let v = v.to_string();
                if v.ends_with(".0") {
                    (&v[0..v.len() - 2]).into()
                } else {
                    v
                }
            }
            Object::Boolean(v) => v.to_string(),
        }
    }

    fn is_truthy(&self, object: &Option<Object>) -> bool {
        match object {
            None => false,
            Some(Object::Boolean(v)) => *v,
            _ => true,
        }
    }

    fn is_equal(&self, a: &Option<Object>, b: &Option<Object>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(a), Some(b)) => a.is_equal(b),
            _ => false,
        }
    }
}

impl Visitor for Interpreter {
    fn visit_literal_expr(&self, expr: Literal) -> Result<Option<Object>, ParseError> {
        Ok(expr.value)
    }

    fn visit_grouping_expr(&self, expr: Grouping) -> Result<Option<Object>, ParseError> {
        return self.evaluate(&expr.expression);
    }

    fn visit_unary_expr(&self, expr: Unary) -> Result<Option<Object>, ParseError> {
        let right = self.evaluate(&expr.right)?;
        match (expr.operator.r#type, right) {
            (TokenType::MINUS, Some(Object::Number(v))) => Ok(Some(Object::Number(-v))),
            (TokenType::BANG, v) => Ok(Some(Object::Boolean(self.is_truthy(&v)))),
            _ => Ok(None), // Unreachable.
        }
    }

    fn visit_binary_expr(&self, expr: Binary) -> Result<Option<Object>, ParseError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match (expr.operator.r#type, left, right) {
            (TokenType::SLASH, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Number(left / right)))
            }
            (TokenType::SLASH, _, _) => Err(ParseError::new(
                expr.operator,
                "Operands must be numbers.".into(),
            )),

            (TokenType::STAR, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Number(left * right)))
            }
            (TokenType::STAR, _, _) => Err(ParseError::new(
                expr.operator,
                "Operands must be numbers.".into(),
            )),

            (TokenType::MINUS, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Number(left - right)))
            }
            (TokenType::MINUS, _, _) => Err(ParseError::new(
                expr.operator,
                "Operands must be numbers.".into(),
            )),

            (TokenType::PLUS, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Number(left + right)))
            }
            (TokenType::PLUS, Some(Object::Str(left)), Some(Object::Str(right))) => {
                Ok(Some(Object::Str(format!("{}{}", left, right))))
            }
            (TokenType::PLUS, _, _) => Err(ParseError::new(
                expr.operator,
                "Operands must be two numbers or two strings.".into(),
            )),

            (TokenType::GREATER, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Boolean(left > right)))
            }
            (TokenType::GREATER_EQUAL, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Boolean(left >= right)))
            }

            (TokenType::LESS, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Boolean(left < right)))
            }
            (TokenType::LESS_EQUAL, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Boolean(left <= right)))
            }

            (TokenType::BANG_EQUAL, a, b) => Ok(Some(Object::Boolean(!self.is_equal(&a, &b)))),
            (TokenType::EQUAL_EQUAL, a, b) => Ok(Some(Object::Boolean(self.is_equal(&a, &b)))),
            _ => Err(ParseError::new(expr.operator, "Unknown error.".into())), // Unreachable.
        }
    }
}
