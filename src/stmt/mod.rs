pub(crate) mod block;
pub(crate) mod class;
pub(crate) mod expression;
pub mod function;
pub mod r#if;
pub(crate) mod print;
pub(crate) mod r#return;
pub(crate) mod var;
pub(crate) mod r#while;

use crate::error::{LoxError, ParseError};
use crate::expr::variable::Variable;
use crate::expr::Expr;
use crate::object::Object;
use crate::stmt::function::Function;
use crate::token::Token;

#[derive(Debug, Clone)]
pub(crate) enum Stmt {
    /// such as a+1;
    Expression(expression::Expression),
    /// such as print 123;
    Print(print::Print),
    /// such as var a = 3; var b;
    Var(var::Var),
    Block(block::Block),
    If(Box<r#if::If>),
    While(Box<r#while::While>),
    Function(Box<Function>),
    Return(r#return::Return),
    Class(class::Class),
}

impl Stmt {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> Result<Option<Object>, LoxError> {
        match self {
            Stmt::Expression(v) => visitor
                .visit_expression_stmt(v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::Print(v) => visitor
                .visit_print_stmt(v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::Var(v) => visitor
                .visit_var_stmt(v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::Block(v) => visitor
                .visit_block_stmt(v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::If(v) => visitor
                .visit_if_stmt(*v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::While(v) => visitor
                .visit_while_stmt(*v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::Function(f) => visitor
                .visit_function_stmt(*f.clone())
                .map(|_| Some(Object::Void)),
            Stmt::Return(v) => visitor
                .visit_return_stmt(v.clone())
                .map(|_| Some(Object::Void)),
            Stmt::Class(_) => todo!(),
        }
    }

    pub fn print(expression: Expr) -> Self {
        Stmt::Print(print::Print { expression })
    }
    pub fn expression(expression: Expr) -> Self {
        Stmt::Expression(expression::Expression { expression })
    }

    pub fn var(token: Token, initializer: Option<Expr>) -> Self {
        Stmt::Var(var::Var {
            name: token,
            initializer,
        })
    }
    pub fn block(statements: Vec<Stmt>) -> Self {
        Stmt::Block(block::Block { statements })
    }
    pub fn r#if(condition: Expr, thenBranch: Stmt, elseBranch: Option<Stmt>) -> Self {
        Stmt::If(Box::new(r#if::If {
            condition,
            then_branch: thenBranch,
            else_branch: elseBranch,
        }))
    }
    pub fn r#while(condition: Expr, body: Stmt) -> Self {
        Stmt::While(Box::new(r#while::While { condition, body }))
    }

    pub fn function(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Stmt::Function(Box::new(function::Function { name, params, body }))
    }

    pub fn r#return(keyword: Token, value: Expr) -> Self {
        Stmt::Return(r#return::Return {
            keyword,
            value: Some(value),
        })
    }

    pub fn class(name: Token, superclass: Option<Variable>, methods: Vec<Function>) -> Self {
        Stmt::Class(class::Class {
            name,
            superclass,
            methods,
        })
    }
}

pub(crate) trait Visitor {
    /// execute expression, ignore result
    fn visit_expression_stmt(&mut self, stmt: expression::Expression) -> Result<(), LoxError>;

    /// print statement
    fn visit_print_stmt(&mut self, stmt: print::Print) -> Result<(), LoxError>;

    /// define var
    fn visit_var_stmt(&mut self, stmt: var::Var) -> Result<(), LoxError>;

    /// execute block
    fn visit_block_stmt(&mut self, stmt: block::Block) -> Result<(), LoxError>;

    /// execute if statement
    fn visit_if_stmt(&mut self, stmt: r#if::If) -> Result<(), LoxError>;

    /// execute while statement
    fn visit_while_stmt(&mut self, stmt: r#while::While) -> Result<(), LoxError>;

    /// define function
    fn visit_function_stmt(&mut self, stmt: function::Function) -> Result<(), LoxError>;

    fn visit_return_stmt(&mut self, stmt: r#return::Return) -> Result<(), LoxError>;

    fn visit_class_stmt(&mut self, stmt: class::Class) -> Result<(), LoxError>;
}
