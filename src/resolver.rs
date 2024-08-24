use std::collections::HashMap;
use std::iter::Map;
use crate::error::LoxError;
use crate::interpreter::Interpreter;
use crate::{expr, stmt};
use crate::expr::assign::Assign;
use crate::expr::binary::Binary;
use crate::expr::call::Call;
use crate::expr::Expr;
use crate::expr::grouping::Grouping;
use crate::expr::literal::Literal;
use crate::expr::logical::Logical;
use crate::expr::unary::Unary;
use crate::expr::variable::Variable;
use crate::lox::Lox;
use crate::object::Object;
use crate::stmt::block::Block;
use crate::stmt::expression::Expression;
use crate::stmt::function::Function;
use crate::stmt::print::Print;
use crate::stmt::r#if::If;
use crate::stmt::r#return::Return;
use crate::stmt::r#while::While;
use crate::stmt::Stmt;
use crate::stmt::var::Var;
use crate::token::Token;

pub(crate) struct Resolver {
    pub interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {

    pub fn new(interpreter: Interpreter) -> Self{
        Self {
            interpreter,
            scopes: vec![]
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
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for i in (0..self.scopes.len()).rev() {
            if let Some(scope) = self.scopes.get(i) {
                if scope.contains_key(&name.lexeme) {
                    self.interpreter.resolve(expr.clone(), self.scopes.len() - 1 - i);
                    return;
                }
            }
        }
    }

    fn resolve_function(&mut self, function: &Function) {
        self.begin_scope();
        for param in &function.params {
            self.declare(param);
            self.define(param);
        }
        self.resolve(&function.body);
        self.end_scope();
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
        self.resolve_function(&stmt);
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: Return) -> Result<(), LoxError> {
        if let Some(expr) = stmt.value {
            self.resolve_expr(&expr);
        }
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
                Lox::error_(&expr.name.clone(), "Can't read local variable in its own initializer.");
            }

        }
        self.resolve_local(&Expr::variable(expr.name.clone()), &expr.name);
        Ok(Some(Object::Void))
    }

    fn visit_assign_expr(&mut self, expr: Assign) -> Result<Option<Object>, LoxError> {
        self.resolve_expr(&expr.value);
        let name = expr.name.clone();
        self.resolve_local(&Expr::Assign(Box::new(expr)), &name);
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
}