mod ast;
mod lexer;
mod parser;
mod token;

use std::error::Error;

use crate::lexer::*;
use crate::parser::*;
use crate::token::*;

fn main() -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string("test.rn")?;
    let mut lexer = Lexer::new(&content);

    loop {
        match lexer.next_token() {
            Token::EOF => break,
            v => println!("{:?}", v),
        }
    }

    Ok(())
}
