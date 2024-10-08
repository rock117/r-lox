use crate::class::ClassType;
use crate::error::LoxError;
use crate::expr::assign::Assign;
use crate::expr::binary::Binary;
use crate::expr::call::Call;
use crate::expr::get::Get;
use crate::expr::grouping::Grouping;
use crate::expr::literal::Literal;
use crate::expr::logical::Logical;
use crate::expr::set::Set;
use crate::expr::super_::Super;
use crate::expr::this::This;
use crate::expr::unary::Unary;
use crate::expr::variable::Variable;
use crate::expr::Expr;
use crate::function::FunctionType;
use crate::function::FunctionType::NONE;
use crate::interpreter::Interpreter;
use crate::lox::Lox;
use crate::object::Object;
use crate::stmt::block::Block;
use crate::stmt::expression::Expression;
use crate::stmt::function::Function;
use crate::stmt::print::Print;
use crate::stmt::r#if::If;
use crate::stmt::r#return::Return;
use crate::stmt::r#while::While;
use crate::stmt::var::Var;
use crate::stmt::{class, Stmt};
use crate::token::Token;
use crate::{expr, function, stmt};
use std::collections::HashMap;
use std::iter::Map;

pub(crate) struct Resolver {
    pub interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    current_class: ClassType,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            current_function: NONE,
            current_class: ClassType::NONE,
            interpreter,
            scopes: vec![],
        }
    }

    pub(crate) fn resolve(&mut self, statements: &Vec<Stmt>) {
        for stmt in statements {
            self.resolve_stmt(stmt);
        }
    }
    fn resolve_stmt(&mut self, stmt: &Stmt) {
        let _ = stmt.accept(self);
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        let _ = expr.accept(self);
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        let Some(scope) = self.scopes.last_mut() else {
            return;
        };
        if (scope.contains_key(&name.lexeme)) {
            Lox::error_(name, "Already variable with this name in this scope.");
        }
        scope.insert(name.lexeme.clone(), false);
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }

    fn resolve_local(&mut self, expr: &mut Expr, name: &Token) {
        for i in (0..self.scopes.len()).rev() {
            if let Some(scope) = self.scopes.get(i) {
                if scope.contains_key(&name.lexeme) {
                    self.interpreter.resolve(expr, self.scopes.len() - 1 - i);
                    return;
                }
            }
        }
    }

    fn resolve_function(&mut self, function: &Function, function_type: FunctionType) {
        let enclosing_function = self.current_function;
        self.current_function = function_type;
        self.begin_scope();
        for param in &function.params {
            self.declare(param);
            self.define(param);
        }
        self.resolve(&function.body);
        self.end_scope();
        self.current_function = enclosing_function;
    }
}

impl stmt::Visitor for Resolver {
    fn visit_expression_stmt(&mut self, stmt: Expression) -> Result<(), LoxError> {
        self.resolve_expr(&stmt.expression);
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: Print) -> Result<(), LoxError> {
        self.resolve_expr(&stmt.expression);
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: Var) -> Result<(), LoxError> {
        self.declare(&stmt.name);
        if let Some(initializer) = stmt.initializer {
            self.resolve_expr(&initializer);
        }
        self.define(&stmt.name);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: Block) -> Result<(), LoxError> {
        self.begin_scope();
        self.resolve(&stmt.statements);
        self.end_scope();
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: If) -> Result<(), LoxError> {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.then_branch);
        if let Some(else_branch) = stmt.else_branch {
            self.resolve_stmt(&else_branch);
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: While) -> Result<(), LoxError> {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.body);
        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: Function) -> Result<(), LoxError> {
        self.declare(&stmt.name);
        self.define(&stmt.name);

        self.resolve_function(&stmt, FunctionType::FUNCTION);
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: Return) -> Result<(), LoxError> {
        if (self.current_function == NONE) {
            Lox::error_(&stmt.keyword, "Can't return from top-level code.");
        }
        if let Some(expr) = stmt.value {
            if self.current_function == FunctionType::INITIALIZER {
                Lox::error_(&stmt.keyword, "Can't return a value from an initializer.");
            }
            self.resolve_expr(&expr);
        }
        Ok(())
    }

    fn visit_class_stmt(&mut self, stmt: class::Class) -> Result<(), LoxError> {
        let enclosing_class = self.current_class;
        self.current_class = ClassType::CLASS;

        self.declare(&stmt.name);
        self.define(&stmt.name);
        if let Some(superclass) = &stmt.superclass {
            if stmt.name.lexeme == superclass.name.lexeme {
                Lox::error_(&superclass.name, "A class can't inherit from itself.");
            }
            self.resolve_expr(&Expr::Variable(superclass.clone()));

            self.begin_scope();
            self.scopes
                .last_mut()
                .map(|map| map.insert("super".into(), true));
        }

        self.begin_scope();
        self.scopes
            .last_mut()
            .map(|map| map.insert("this".into(), true));
        for method in stmt.methods {
            let mut declaration = FunctionType::METHOD;
            if method.name.lexeme == "init" {
                declaration = FunctionType::INITIALIZER;
            }
            self.resolve_function(&method, declaration);
        }
        self.end_scope();

        if stmt.superclass.is_some() {
            self.end_scope();
        }
        self.current_class = enclosing_class;
        Ok(())
    }
}

impl expr::Visitor for Resolver {
    fn visit_literal_expr(&self, expr: Literal) -> Result<Option<Object>, LoxError> {
        Ok(None)
    }

    fn visit_grouping_expr(&mut self, expr: Grouping) -> Result<Option<Object>, LoxError> {
        self.resolve_expr(&expr.expression);
        Ok(None)
    }

    fn visit_unary_expr(&mut self, expr: Unary) -> Result<Option<Object>, LoxError> {
        self.resolve_expr(&expr.right);
        Ok(None)
    }

    fn visit_binary_expr(&mut self, expr: Binary) -> Result<Option<Object>, LoxError> {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
        Ok(None)
    }

    fn visit_variable_expr(&mut self, expr: Variable) -> Result<Option<Object>, LoxError> {
        println!("visit_variable_expr scope: {:?}", self.scopes);
        if !self.scopes.is_empty() {
            let exist = self.scopes.last().map(|last| last.get(&expr.name.lexeme));
            if let Some(Some(&false)) = exist {
                Lox::error_(
                    &expr.name.clone(),
                    "Can't read local variable in its own initializer.",
                );
            }
        }
        self.resolve_local(&mut Expr::variable(expr.name.clone()), &expr.name);
        Ok(Some(Object::Void))
    }

    fn visit_assign_expr(&mut self, expr: Assign) -> Result<Option<Object>, LoxError> {
        self.resolve_expr(&expr.value);
        let name = expr.name.clone();
        self.resolve_local(&mut Expr::Assign(Box::new(expr)), &name);
        Ok(Some(Object::Void))
    }

    fn visit_logical_expr(&mut self, expr: Logical) -> Result<Option<Object>, LoxError> {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
        Ok(None)
    }

    fn visit_call_expr(&mut self, expr: Call) -> Result<Option<Object>, LoxError> {
        self.resolve_expr(&expr.callee);
        for argument in &expr.arguments {
            self.resolve_expr(argument);
        }
        Ok(Some(Object::Void))
    }

    fn visit_get_expr(&mut self, expr: Get) -> Result<Option<Object>, LoxError> {
        self.resolve_expr(&expr.object);
        Ok(Some(Object::Void))
    }

    fn visit_set_expr(&mut self, expr: Set) -> Result<Option<Object>, LoxError> {
        self.resolve_expr(&expr.value);
        self.resolve_expr(&expr.object);
        Ok(Some(Object::Void))
    }

    fn visit_this_expr(&mut self, mut expr: This) -> Result<Option<Object>, LoxError> {
        if ClassType::NONE == self.current_class {
            Lox::error_(&expr.keyword, "Can't use 'this' outside of a class.");
            return Ok(Some(Object::Void));
        }
        self.resolve_local(&mut Expr::this(expr.keyword.clone()), &expr.keyword);
        Ok(Some(Object::Void))
    }

    fn visit_super_expr(&mut self, expr: Super) -> Result<Option<Object>, LoxError> {
        if self.current_class == ClassType::NONE {
            Lox::error_(&expr.keyword, "Can't use 'super' outside of a class.");
        } else if self.current_class != ClassType::SUBCLASS {
            Lox::error_(
                &expr.keyword,
                "Can't use 'super' in a class with no superclass.",
            );
        }
        self.resolve_local(
            &mut Expr::super_(expr.keyword.clone(), expr.method),
            &expr.keyword,
        );
        Ok(Some(Object::Void))
    }
}
