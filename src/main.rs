#![feature(once_cell)]
#![feature(char_indices_offset)]

use crate::ast::Ast;
use std::error::Error;

use crate::transpiler::{Lexer, ParseErrorInfo, Parser, Token};

mod ast;
mod transpiler;
mod util;

fn main() -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string("test.rn")?;
    let mut parser = Parser::new(&content);

    match parser.parse() {
        Ok(v) => println!("{:#?}", v),
        Err(e) => eprintln!("ERROR: {}", e),
    }

    Ok(())
}
