use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::environment::Environment;
use crate::error::{LoxError, ParseError, Return};
use crate::expr::binary::Binary;
use crate::expr::call::Call;
use crate::expr::grouping::Grouping;
use crate::expr::literal::Literal;
use crate::expr::unary::Unary;
use crate::expr::{assign, logical, variable, Expr};
use crate::function;
use crate::function::lox_function::LoxFunction;
use crate::function::native_function::NativeFunction;
use crate::lox::Lox;
use crate::object::Object;
use crate::stmt::function::Function;
use crate::stmt::print::Print;
use crate::stmt::{block, expression, r#if, r#return, r#while, Stmt};
use crate::token::token_type::TokenType;
use crate::{expr, stmt};

pub(crate) struct Interpreter {
    globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Rc::new(RefCell::new(Environment::new()));
        globals.borrow_mut().define(
            "clock".into(),
            Some(Object::Function(Box::new(
                function::LoxCallable::NativeFunction(NativeFunction::clock()),
            ))),
        );
        let environment = globals.clone();

        Self {
            globals,
            environment,
        }
    }
    pub fn interpret(&mut self, statements: &[Stmt]) {
        for stmt in statements {
            let result = self.execute(stmt);
            if let Err(e) = result {
                Lox::runtime_error(e);
                return;
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Option<Object>, LoxError> {
        return expr.accept(self);
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<Option<Object>, LoxError> {
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
            Object::Function(f) => f.to_string(),
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

    pub(crate) fn execute_block(
        &mut self,
        statements: Vec<Stmt>,
        environment: Environment,
    ) -> Result<(), LoxError> {
        let previous = self.environment.clone();
        self.environment = Rc::new(RefCell::new(environment));
        for stmt in statements {
            if let Err(e) = self.execute(&stmt) {
                self.environment = previous;
                return Err(e);
            }
        }
        self.environment = previous;
        Ok(())
    }
}

impl expr::Visitor for Interpreter {
    fn visit_literal_expr(&self, expr: Literal) -> Result<Option<Object>, LoxError> {
        Ok(expr.value)
    }

    fn visit_grouping_expr(&mut self, expr: Grouping) -> Result<Option<Object>, LoxError> {
        return self.evaluate(&expr.expression);
    }

    fn visit_unary_expr(&mut self, expr: Unary) -> Result<Option<Object>, LoxError> {
        let right = self.evaluate(&expr.right)?;
        match (expr.operator.r#type, right) {
            (TokenType::MINUS, Some(Object::Number(v))) => Ok(Some(Object::Number(-v))),
            (TokenType::BANG, v) => Ok(Some(Object::Boolean(self.is_truthy(&v)))),
            _ => Ok(None), // Unreachable.
        }
    }

    fn visit_binary_expr(&mut self, expr: Binary) -> Result<Option<Object>, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match (expr.operator.r#type, left, right) {
            (TokenType::SLASH, Some(Object::Number(left)), Some(Object::Number(0f64))) => Err(
                LoxError::new_parse_error(expr.operator, "Arithmetic Error: / by zero".into()),
            ),
            (TokenType::SLASH, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Number(left / right)))
            }
            (TokenType::SLASH, _, _) => Err(LoxError::new_parse_error(
                expr.operator,
                "Operands must be numbers.".into(),
            )),

            (TokenType::STAR, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Number(left * right)))
            }
            (TokenType::STAR, _, _) => Err(LoxError::new_parse_error(
                expr.operator,
                "Operands must be numbers.".into(),
            )),

            (TokenType::MINUS, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Number(left - right)))
            }
            (TokenType::MINUS, _, _) => Err(LoxError::new_parse_error(
                expr.operator,
                "Operands must be numbers.".into(),
            )),

            (TokenType::PLUS, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Number(left + right)))
            }
            (TokenType::PLUS, Some(Object::Str(left)), Some(Object::Str(right))) => {
                Ok(Some(Object::Str(format!("{}{}", left, right))))
            }
            (TokenType::PLUS, Some(Object::Number(left)), Some(Object::Str(right))) => {
                Ok(Some(Object::Str(format!("{}{}", left, right))))
            }
            (TokenType::PLUS, Some(Object::Str(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Str(format!("{}{}", left, right))))
            }
            (TokenType::PLUS, _, _) => Err(LoxError::new_parse_error(
                expr.operator,
                "Operands must be two numbers/strings.".into(),
            )),

            (TokenType::GREATER, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Boolean(left > right)))
            }
            (TokenType::GREATER, _, _) => Err(LoxError::new_parse_error(
                expr.operator,
                "Operands must be numbers.".into(),
            )),

            (TokenType::GREATER_EQUAL, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Boolean(left >= right)))
            }
            (TokenType::GREATER_EQUAL, _, _) => Err(LoxError::new_parse_error(
                expr.operator,
                "Operands must be numbers.".into(),
            )),

            (TokenType::LESS, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Boolean(left < right)))
            }
            (TokenType::LESS, _, _) => Err(LoxError::new_parse_error(
                expr.operator,
                "Operands must be numbers.".into(),
            )),

            (TokenType::LESS_EQUAL, Some(Object::Number(left)), Some(Object::Number(right))) => {
                Ok(Some(Object::Boolean(left <= right)))
            }
            (TokenType::LESS_EQUAL, _, _) => Err(LoxError::new_parse_error(
                expr.operator,
                "Operands must be numbers.".into(),
            )),

            (TokenType::BANG_EQUAL, a, b) => Ok(Some(Object::Boolean(!self.is_equal(&a, &b)))),
            (TokenType::EQUAL_EQUAL, a, b) => Ok(Some(Object::Boolean(self.is_equal(&a, &b)))),
            _ => Err(LoxError::new_parse_error(
                expr.operator,
                "Unknown error.".into(),
            )), // Unreachable.
        }
    }

    fn visit_variable_expr(&self, expr: variable::Variable) -> Result<Option<Object>, LoxError> {
        self.environment.borrow().get(&expr.name)
    }

    fn visit_assign_expr(&mut self, expr: assign::Assign) -> Result<Option<Object>, LoxError> {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow_mut()
            .assign(&expr.name, value.clone())?;
        Ok(value)
    }

    fn visit_logical_expr(&mut self, expr: logical::Logical) -> Result<Option<Object>, LoxError> {
        let left = self.evaluate(&expr.left)?;
        if expr.operator.r#type == TokenType::OR {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else {
            if !self.is_truthy(&left) {
                return Ok(left);
            }
        }
        return self.evaluate(&expr.right);
    }

    fn visit_call_expr(&mut self, expr: Call) -> Result<Option<Object>, LoxError> {
        let callee = self.evaluate(&expr.callee)?;
        let mut arguments = vec![];
        for argument in expr.arguments {
            arguments.push(self.evaluate(&argument)?);
        }

        let Some(callee) = callee else {
            return Err(LoxError::new_parse_error(
                expr.paren,
                "Can only call functions and classes.".into(),
            ));
        };

        let Object::Function(function) = callee else {
            return Err(LoxError::new_parse_error(
                expr.paren,
                "Can only call functions and classes.".into(),
            ));
        };

        if (arguments.len() != function.arity()) {
            return Err(LoxError::new_parse_error(
                expr.paren,
                format!(
                    "Expected {} arguments but got {}",
                    function.arity(),
                    arguments.len()
                ),
            ));
        }
        function.call(self, arguments)
    }
}

impl stmt::Visitor for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: expression::Expression) -> Result<(), LoxError> {
        self.evaluate(&stmt.expression).map(|_| ())
    }

    fn visit_print_stmt(&mut self, stmt: Print) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", self.stringify(value));
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: stmt::var::Var) -> Result<(), LoxError> {
        let value = if let Some(initializer) = stmt.initializer {
            self.evaluate(&initializer)?
        } else {
            None
        };
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme, value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: block::Block) -> Result<(), LoxError> {
        self.execute_block(
            stmt.statements,
            Environment::new_from_enclosing(self.environment.clone()),
        )
    }

    fn visit_if_stmt(&mut self, stmt: r#if::If) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.condition)?;
        if self.is_truthy(&value) {
            self.execute(&stmt.thenBranch)?;
            return Ok(());
        }
        if let Some(elseBranch) = stmt.elseBranch {
            self.execute(&elseBranch)?;
            return Ok(());
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: r#while::While) -> Result<(), LoxError> {
        let mut value = self.evaluate(&stmt.condition)?;
        while self.is_truthy(&value) {
            self.execute(&stmt.body)?; // TODO fix bug, is_truthy always true/false
            value = self.evaluate(&stmt.condition)?;
        }
        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: Function) -> Result<(), LoxError> {
        let environment = self.environment.clone();
        let name = stmt.name.lexeme.clone();
        let function = LoxFunction {
            declaration: stmt,
            closure: environment,
        };
        let function = Box::new(function::LoxCallable::LoxFunction(function));
        self.environment
            .borrow_mut()
            .define(name, Some(Object::Function(function)));
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: r#return::Return) -> Result<(), LoxError> {
        let value = if let Some(value) = stmt.value {
            self.evaluate(&value)?
        } else {
            None
        };
        Err(LoxError::ReturnError(Return { value }))
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
