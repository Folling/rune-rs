use crate::ast::structural::FuncDecl;
use crate::ast::{Ident, Node};
use crate::transpiler::ParseError;
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

    fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        let mut path = Vec::with_capacity(16);

        loop {
            path.push(Ident::parse(lexer)?);

            match lexer.cur_next() {
                Some(Token::Special("::")) => continue,
                Some(Token::Special(";")) => break,
                None => return Err(ParseError::PrematureEOF),
                Some(v) => {
                    return Err(ParseError::InvalidToken {
                        got: v,
                        expected: vec![Token::Special("::"), Token::Special(";")],
                    })
                }
            }
        }

        Ok(UseDecl { path })
    }
}
