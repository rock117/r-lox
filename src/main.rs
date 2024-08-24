use std::error::Error;
use std::io::Read;

use crate::environment::Environment;
use anyhow::anyhow;

use crate::lox::Lox;
use crate::object::Object;
use crate::token::token_type::TokenType;
use crate::token::Token;

mod environment;
mod error;
mod expr;
mod function;
mod interpreter;
mod lox;
mod object;
mod parser;
mod resolver;
pub(crate) mod scanner;
pub(crate) mod stmt;
pub(crate) mod token;

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
        Lox::run_file(r#"C:\rock\coding\code\my\rust\r-lox\tmp\a.lox"#);
        // Lox::run_prompt();
    }

    Ok(())
}
