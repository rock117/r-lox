use crate::expr::ast_printer::AstPrinter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::token_type::TokenType;
use crate::token::Token;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::interpreter::Interpreter;

pub struct Lox {
    interpreter: Interpreter,
}

static HAD_ERROR: AtomicBool = AtomicBool::new(false);
static interpreter: Interpreter = Interpreter::new();

impl Lox {

    pub(crate) fn run_file(path: &str) -> anyhow::Result<()> {
        let source_code = std::fs::read_to_string(path)?;
        Lox::run(source_code);
        if HAD_ERROR.load(Ordering::Relaxed) {
            std::process::exit(65);
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
                    Lox::run(line);
                    HAD_ERROR.store(false, Ordering::SeqCst);
                }
                Err(e) => return,
            }
        }
    }

    fn run(source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();
        // Stop if there was a syntax error.
        if HAD_ERROR.load(Ordering::SeqCst) {
            return;
        }
        if let Some(exp) = expression {
            println!("{}", AstPrinter::new().print(exp))
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
}
