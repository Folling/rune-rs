use std::error::Error;

use crate::transpiler::{Lexer, Parser, Token};

mod ast;
mod transpiler;

fn main() -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string("test.rn")?;
    let mut parser = Parser::new(&content);

    let ast = parser.parse();

    println!("{:#?}", ast);

    Ok(())
}
