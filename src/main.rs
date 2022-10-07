#![feature(once_cell)]
#![feature(char_indices_offset)]

use std::error::Error;

use crate::transpiler::{Lexer, Parser, Token};

mod ast;
mod transpiler;
mod util;

fn main() -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string("test.rn")?;
    let mut parser = Parser::new(&content);

    match parser.parse() {
        Ok(val) => println!("{:#?}", val),
        Err(e) => eprintln!("ERROR: {}", e),
    }

    Ok(())
}
