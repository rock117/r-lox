use std::error::Error;
use std::io::Read;

use anyhow::anyhow;

use crate::lox::Lox;

mod lox;
mod scanner;
mod token;

fn main() -> anyhow::Result<()> {
    let args = std::env::args();
    if args.len() > 1 {
        println!("Usage: jlox [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
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
