use crate::scanner::Scanner;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Lox;
static HAD_ERROR: AtomicBool = AtomicBool::new(false);

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
        // For now, just print the tokens.
        for token in tokens {
            println!("{:?}", token);
        }
    }

    pub(crate) fn error(line: usize, message: &str) {
        Lox::report(line, "", message);
    }

    fn report(line: usize, r#where: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, r#where, message);
        HAD_ERROR.store(true, Ordering::Relaxed);
    }
}
