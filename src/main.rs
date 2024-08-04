use std::error::Error;
use std::io::Read;

use anyhow::anyhow;

use crate::lox::Lox;

mod environment;
mod error;
mod expr;
mod interpreter;
mod lox;
mod object;
mod parser;
mod scanner;
mod stmt;
mod token;

fn main() -> anyhow::Result<()> {
    let args = std::env::args();
    println!("args len: {}, {:?}", args.len(), args);
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        Lox::run_file(
            args.into_iter()
                .next()
                .ok_or(anyhow!("no arg provide"))?
                .as_str(),
        )?;
    } else {
        Lox::run_prompt();
    }
    Ok(())
}
