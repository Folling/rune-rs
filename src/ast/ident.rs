use std::error::Error;

use crate::ast::Node;
use crate::transpiler::ParseError;
use crate::{Lexer, Token};

#[derive(Debug)]
pub struct Ident<'a> {
    pub(crate) raw: bool,
    pub(crate) name: &'a str,
}

impl<'a> Node<'a> for Ident<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        true
    }
}

impl<'a> Ident<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        match lexer.cur_next() {
            Some(Token::Special(v)) => Err(ParseError::InvalidToken {
                got: Token::Special(v),
                expected: vec![Token::Textual("any text")],
            }),
            // FIXME, this needs to be separate, r# will be parsed as [r, #]
            Some(Token::Textual(v)) => Ok(Ident {
                raw: v.starts_with("r#"),
                name: v,
            }),
            None => Err(ParseError::PrematureEOF),
        }
    }
}
