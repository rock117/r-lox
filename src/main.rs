use std::error::Error;
use std::io::Read;
use anyhow::anyhow;

use crate::scanner::Scanner;

mod scanner;
mod token;

fn main() -> anyhow::Result<()>{
    let args = std::env::args();
    if args.len() > 1 {
        println!("Usage: jlox [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(args.into_iter().next().ok_or(anyhow!("no arg provide"))?.as_str())?;
    } else {
        run_prompt();
    }
    Ok(())
}

fn run_file(path: &str) -> anyhow::Result<()>{
    let source_code = std::fs::read_to_string(path)?;
    run(source_code);
    Ok(())
}

fn run_prompt(){
    loop {
        print!("> ");
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(0) => return,
            Ok(n) => run(line),
            Err(e) => return
        }
    }
}

fn run(source: String) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    // For now, just print the tokens.
    for token in tokens {
        println!("{:?}", token);
    }
}