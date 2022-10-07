use crate::ast::Node;
use crate::transpiler::{ExpectedToken, ParseErr};
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
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseErr<'a>>
    where
        Self: Sized,
    {
        match lexer.cur_next() {
            None => Err(ParseErr::PrematureEOF),
            // FIXME, this needs to be separate, r# will be parsed as [r, #]
            Some((Token::Textual(val), _)) => Ok(Ident {
                raw: val.starts_with("r#"),
                name: val,
            }),
            Some((val, _)) => Err(ParseErr::InvalidToken {
                got: val,
                expected: vec![ExpectedToken::Textual { regex: "any text" }],
            }),
        }
    }
}
