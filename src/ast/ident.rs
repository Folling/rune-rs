use std::error::Error;

use crate::ast::Node;
use crate::transpiler::{ExpectedToken, ParseError};
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
            None => Err(ParseError::PrematureEOF),
            // FIXME, this needs to be separate, r# will be parsed as [r, #]
            Some(Token::Textual { value, .. }) => Ok(Ident {
                raw: value.starts_with("r#"),
                name: v,
            }),
            Some(v) => Err(ParseError::InvalidToken {
                got: v,
                expected: vec![ExpectedToken::Textual { regex: "any text" }],
            }),
        }
    }
}
