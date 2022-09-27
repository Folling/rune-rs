use crate::ast::structural::FuncDecl;
use crate::ast::{Ident, Node};
use crate::transpiler::{ExpectedToken, ParseError};
use crate::{Lexer, Token};

#[derive(Debug)]
pub struct UseDecl<'a> {
    path: Vec<Ident<'a>>,
}

impl<'a> Node<'a> for UseDecl<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        self.path.iter().all(Node::valid)
    }
}

impl<'a> UseDecl {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        let mut path = Vec::with_capacity(16);

        loop {
            path.push(Ident::parse(lexer)?);

            match lexer.cur_next() {
                Some(Token::Special { value: "::", .. }) => continue,
                Some(Token::Special { value: ";", .. }) => break,
                None => return Err(ParseError::PrematureEOF),
                Some(v) => {
                    return Err(ParseError::InvalidToken {
                        got: v,
                        expected: vec![ExpectedToken::Special { regex: "::" }, ExpectedToken::Special { regex: ";" }],
                    })
                }
            }
        }

        Ok(UseDecl { path })
    }
}
