use std::fmt::Display;

use crate::error::ParseError;
use crate::expr::binary::Binary;
use crate::expr::grouping::Grouping;
use crate::expr::literal::Literal;
use crate::expr::unary::Unary;
use crate::expr::Expr;
use crate::lox::Lox;
use crate::object::Object;
use crate::stmt::print::Print;
use crate::stmt::{expression, Stmt};
use crate::token::token_type::TokenType;
use crate::{expr, stmt};

pub(crate) struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }
    pub fn interpret(&self, statements: &[Stmt]) {
        for stmt in statements {
            let result = self.execute(stmt);
            if let Err(e) = result {
                Lox::runtime_error(e);
                return;
            }
        }
    }

    fn evaluate(&self, expr: &Expr) -> Result<Option<Object>, ParseError> {
        return expr.accept(self);
    }

    fn execute(&self, stmt: &Stmt) -> Result<Option<Object>, ParseError> {
        stmt.accept(self)
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
            Object::Void => "".into(),
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

impl expr::Visitor for Interpreter {
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
            (TokenType::SLASH, Some(Object::Number(left)), Some(Object::Number(0f64))) => Err(
                ParseError::new(expr.operator, "Arithmetic Error: / by zero".into()),
            ),
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
            (TokenType::GREATER, _, _) => Err(ParseError::new(
                expr.operator,
                "Operands must be numbers.".into(),
            )),

            (TokenType::GREATER_EQUAL, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Boolean(left >= right)))
            }
            (TokenType::GREATER_EQUAL, _, _) => Err(ParseError::new(
                expr.operator,
                "Operands must be numbers.".into(),
            )),

            (TokenType::LESS, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Boolean(left < right)))
            }
            (TokenType::LESS, _, _) => Err(ParseError::new(
                expr.operator,
                "Operands must be numbers.".into(),
            )),

            (TokenType::LESS_EQUAL, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Boolean(left <= right)))
            }
            (TokenType::LESS_EQUAL, _, _) => Err(ParseError::new(
                expr.operator,
                "Operands must be numbers.".into(),
            )),

            (TokenType::BANG_EQUAL, a, b) => Ok(Some(Object::Boolean(!self.is_equal(&a, &b)))),
            (TokenType::EQUAL_EQUAL, a, b) => Ok(Some(Object::Boolean(self.is_equal(&a, &b)))),
            _ => Err(ParseError::new(expr.operator, "Unknown error.".into())), // Unreachable.
        }
    }
}

impl stmt::Visitor for Interpreter {
    fn visit_expression_stmt(&self, stmt: expression::Expression) -> Result<(), ParseError> {
        self.evaluate(&stmt.expression).map(|_| ())
    }

    fn visit_print_stmt(&self, stmt: Print) -> Result<(), ParseError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{:?}", self.stringify(value));
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::interpreter::Interpreter;
    use crate::parser::Parser;
    use crate::scanner::Scanner;

    // #[test]
    // fn test_evaluate_success() {
    //     let tokens = Scanner::new("1 + 2".into()).scan_tokens();
    //     let mut parser = Parser::new(tokens);
    //     let exp = parser.parse().unwrap();
    //     let interceptor = Interpreter::new();
    //     let value = interceptor.evaluate(&exp).unwrap();
    //     assert_eq!("3", interceptor.stringify(value))
    // }
    //
    // #[test]
    // fn test_evaluate_failed() {
    //     let tokens = Scanner::new("1 + \"a\"".into()).scan_tokens();
    //     let mut parser = Parser::new(tokens);
    //     let exp = parser.parse().unwrap();
    //     let interceptor = Interpreter::new();
    //     let value = interceptor.evaluate(&exp);
    //     assert_eq!(true, value.is_err())
    // }
}
