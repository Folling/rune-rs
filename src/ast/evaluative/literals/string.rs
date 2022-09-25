use crate::ast::Node;
use crate::transpiler::{util, ParseError};
use crate::{Lexer, Token};

#[derive(Debug)]
pub struct StringLit<'a> {
    value: &'a str,
}

impl<'a> Node<'a> for StringLit<'a> {
    fn generate(&self, content: &mut String) {
        todo!()
    }

    fn valid(&self) -> bool {
        true
    }
}

impl<'a> StringLit<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        let value = match lexer.cur_next() {
            None => return Err(ParseError::PrematureEOF),
            Some(Token::Textual(str)) => str,
            Some(v) => {
                return Err(ParseError::InvalidToken {
                    got: v,
                    expected: vec![Token::Textual("any valid text")],
                })
            }
        };

        util::exp_cur_next_tok(lexer, Token::Textual("\""))?;

        Ok(StringLit { value })
    }
}
