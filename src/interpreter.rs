use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::mem::discriminant;
use std::rc::Rc;

use crate::class::LoxClass;
use crate::environment::Environment;
use crate::error::{LoxError, ParseError, Return};
use crate::expr::binary::Binary;
use crate::expr::call::Call;
use crate::expr::grouping::Grouping;
use crate::expr::literal::Literal;
use crate::expr::set::Set;
use crate::expr::this::This;
use crate::expr::unary::Unary;
use crate::expr::{assign, get, logical, variable, Expr};
use crate::function::lox_function::LoxFunction;
use crate::function::LoxCallable::NativeFunction;
use crate::function::{native_function, LoxCallable};
use crate::instance::LoxInstance;
use crate::lox::Lox;
use crate::object::Object;
use crate::stmt::class::Class;
use crate::stmt::function::Function;
use crate::stmt::print::Print;
use crate::stmt::{block, expression, r#if, r#return, r#while, Stmt};
use crate::token::token_type::TokenType;
use crate::token::Token;
use crate::{expr, function, stmt};

pub(crate) struct Interpreter {
    globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
    locals: HashMap<String, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Rc::new(RefCell::new(Environment::new()));
        globals.borrow_mut().define(
            "clock".into(),
            Some(Object::Function(Box::new(NativeFunction(
                native_function::NativeFunction::clock(),
            )))),
        );
        let environment = globals.clone();

        Self {
            locals: HashMap::new(),
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
            Object::Class(class) => class.to_string(),
            Object::Instance(instance) => instance.to_string(),
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

    /// save distance info to expr
    pub(crate) fn resolve(&mut self, expr: &mut Expr, depth: usize) {
        if let Expr::Variable(v) = expr {
            v.distance = Some(depth)
        }
        // self.locals.insert(expr.id(), depth); copy from book
    }

    fn lookup_variable(&mut self, name: Token, expr: &Expr) -> Result<Option<Object>, LoxError> {
        //   let distance = self.locals.get(&expr.id()); // TODO bug clone will change expr id
        let distance = expr.distance();
        println!("{}, distance is {:?}", name, distance);
        if let Some(distance) = distance {
            match self.environment.borrow().get_at(distance, &name.lexeme) {
                None => Ok(None),
                Some(v) => Ok(v.clone()),
            }
        } else {
            self.globals.borrow().get(&name)
        }
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

    fn visit_variable_expr(
        &mut self,
        expr: variable::Variable,
    ) -> Result<Option<Object>, LoxError> {
        self.lookup_variable(expr.name.clone(), &Expr::variable(expr.name))
    }

    fn visit_assign_expr(&mut self, expr: assign::Assign) -> Result<Option<Object>, LoxError> {
        let value = self.evaluate(&expr.value)?;
        let distance = expr.distance; // TODO self.locals.get(&expr);
        match distance {
            Some(distance) => {
                self.environment
                    .borrow_mut()
                    .assign_at(distance, &expr.name, value.clone())
            }
            None => self
                .globals
                .borrow_mut()
                .assign(&expr.name, value.clone())?,
        }
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

    fn visit_get_expr(&mut self, expr: get::Get) -> Result<Option<Object>, LoxError> {
        let object = self.evaluate(&expr.object)?;
        if let Some(Object::Instance(object)) = object {
            return object.get(expr.name).map(|v| Some(v));
        }
        Err(LoxError::new_parse_error(
            expr.name,
            "Only instances have properties.".into(),
        ))
    }

    fn visit_set_expr(&mut self, expr: Set) -> Result<Option<Object>, LoxError> {
        let object = self.evaluate(&expr.object)?;

        let Some(Object::Instance(mut object)) = object else {
            return Err(LoxError::new_parse_error(
                expr.name,
                "Only instances have fields.".into(),
            ));
        };

        let value = self.evaluate(&expr.value)?;
        if let Some(value) = value.clone() {
            object.set(&expr.name, value)?;
        };
        Ok(value)
    }

    fn visit_this_expr(&mut self, expr: This) -> Result<Option<Object>, LoxError> {
        self.lookup_variable(expr.keyword.clone(), &Expr::this(expr.keyword))
    }

    fn visit_super_expr(&mut self, expr: expr::super_::Super) -> Result<Option<Object>, LoxError> {
        let distance = self.locals.get(""); // expr
        let Some(distance) = distance else {
            return Err(LoxError::new_parse_error(
                expr.method.clone(),
                "distance not exist".into(),
            ));
        };
        let distance = *distance;
        let superclass = self.environment.borrow().get_at(distance, "super"); // (LoxClass)
        let object = self.environment.borrow().get_at(distance - 1, "this"); // LoxInstance

        let (Some(Some(Object::Class(superclass))), Some(Some(Object::Instance(object)))) =
            (superclass, object)
        else {
            return Err(LoxError::new_parse_error(
                expr.method.clone(),
                format!("Undefined property '{}'", expr.method.lexeme),
            ));
        };
        let method = superclass.find_method(&expr.method.lexeme);
        match method {
            Some(method) => Ok(Some(Object::Function(Box::new(LoxCallable::LoxFunction(
                method.bind(object),
            ))))),
            None => Err(LoxError::new_parse_error(
                expr.method.clone(),
                format!("Undefined property '{}'", expr.method.lexeme),
            )),
        }
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
            self.execute(&stmt.then_branch)?;
            return Ok(());
        }
        if let Some(elseBranch) = stmt.else_branch {
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
            is_initializer: false,
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

    fn visit_class_stmt(&mut self, stmt: Class) -> Result<(), LoxError> {
        let superclass = if let Some(ref superclass) = stmt.superclass {
            let object = self.evaluate(&Expr::Variable(superclass.clone()))?;
            let Some(Object::Class(class)) = object else {
                return Err(LoxError::new_parse_error(
                    superclass.name.clone(),
                    "Superclass must be a class.".into(),
                ));
            };
            Some(class)
        } else {
            None
        };

        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), None);

        if stmt.superclass.is_some() {
            self.environment = Rc::new(RefCell::new(Environment::new_from_enclosing(
                self.environment.clone(),
            )));
            self.environment
                .borrow_mut()
                .define("super".into(), superclass.clone().map(|v| Object::Class(v)));
        }

        let mut methods = HashMap::new();
        for method in &stmt.methods {
            let function = LoxFunction {
                declaration: method.clone(),
                closure: self.environment.clone(),
                is_initializer: method.name.lexeme == "init",
            };
            methods.insert(method.name.lexeme.clone(), function);
        }

        let klass = LoxClass::new(stmt.name.lexeme.clone(), superclass.clone(), methods);

        if superclass.is_some() {
            if let Some(enclosing) = self.environment.clone().borrow().enclosing.clone() {
                self.environment = enclosing;
            }
        }

        self.environment
            .borrow_mut()
            .assign(&stmt.name, Some(Object::Class(klass)))?;
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
