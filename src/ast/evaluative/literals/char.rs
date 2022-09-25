use crate::ast::Node;
use crate::transpiler::ParseError::InvalidLiteral;
use crate::transpiler::{util, ParseError};
use crate::{Lexer, Token};
use std::num::ParseIntError;

#[derive(Debug)]
pub struct CharLit<'a> {
    value: &'a str,
}

impl<'a> Node<'a> for CharLit<'a> {
    fn generate(&self, content: &mut String) {
        todo!()
    }

    fn valid(&self) -> bool {
        let mut chars = self.value.char_indices();

        match (chars.next(), chars.next()) {
            (None, None) => false,
            (Some(_), None) => true,
            (Some((_, '\\')), Some((_, 'u'))) => {
                if chars.next().is_none() {
                    return false;
                }

                let str = &self.value[chars.offset()..];

                if u32::from_str_radix(str, 16).ok().and_then(char::from_u32).is_none() {
                    return false;
                }
            }
            (Some((_, '\\')), Some(_)) => true,
            _ => false,
        }
    }
}

impl<'a> CharLit<'a> {
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
                    expected: vec![Token::Textual("any valid char")],
                })
            }
        };

        util::exp_cur_next_tok(lexer, Token::Textual("'"))?;

        Ok(CharLit { value })
    }
}
