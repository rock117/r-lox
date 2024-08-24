use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::error::{LoxError, ParseError};
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::resolver::Resolver;
use crate::scanner::Scanner;
use crate::token::token_type::TokenType;
use crate::token::Token;

pub struct Lox;

static HAD_ERROR: AtomicBool = AtomicBool::new(false);
static HAD_RUNTIME_ERROR: AtomicBool = AtomicBool::new(false);

impl Lox {
    pub(crate) fn run_file(path: &str) -> anyhow::Result<()> {
        let source_code = std::fs::read_to_string(path)?;
        Lox::run(Interpreter::new(), source_code);
        if HAD_ERROR.load(Ordering::Relaxed) {
            std::process::exit(65);
        }
        if HAD_RUNTIME_ERROR.load(Ordering::Relaxed) {
            std::process::exit(70)
        }
        Ok(())
    }

    pub(crate) fn run_prompt() {
        loop {
            print!("> ");
            let mut line = String::new();
            match std::io::stdin().read_line(&mut line) {
                Ok(0) => return,
                Ok(n) => {
                    Lox::run(Interpreter::new(), line);
                    HAD_ERROR.store(false, Ordering::SeqCst);
                }
                Err(e) => return,
            }
        }
    }

    fn run(mut interpreter: Interpreter, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse();

        if HAD_ERROR.load(Ordering::Relaxed) {
            return;
        }

        if let Ok(stmts) = stmts {
           let mut resolver = Resolver::new(interpreter);
           resolver.resolve(&stmts);
           resolver.interpreter.interpret(&stmts);
        }
    }

    pub(crate) fn error(line: usize, message: &str) {
        Lox::report(line, "", message);
    }

    fn report(line: usize, r#where: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, r#where, message);
        HAD_ERROR.store(true, Ordering::Relaxed);
    }

    pub(crate) fn error_(token: &Token, message: &str) {
        if token.r#type == TokenType::EOF {
            Lox::report(token.line, " at end", message);
        } else {
            Lox::report(token.line, &format!(" at '{}'", token.lexeme), message);
        }
    }

    pub(crate) fn runtime_error(error: LoxError) {
        match error {
            LoxError::ParseError(e) => eprintln!("{}\n[line {} ]", e.message, e.token.line),
            LoxError::ReturnError(e) => eprintln!("ReturnError: {:?}", e),
        }

        HAD_RUNTIME_ERROR.store(true, Ordering::SeqCst);
    }
}
