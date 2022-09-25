use crate::ast::{Line, Node};
use crate::transpiler::{util, ParseError};
use crate::{Lexer, Token};

#[derive(Debug)]
pub struct Block<'a> {
    pub(crate) lines: Vec<Line<'a>>,
}

impl<'a> Node<'a> for Block<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        true
    }
}

impl<'a> Block<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        util::exp_cur_next_tok(lexer, Token::Special("{"))?;

        let mut lines = vec![];

        loop {
            match lexer.cur() {
                None => return Err(ParseError::PrematureEOF),
                Some(Token::Special(";")) => {
                    lexer.next_cur();
                    continue;
                }
                Some(Token::Special("}")) => {
                    lexer.next_cur();
                    break;
                }
                v => lines.push(Line::parse(lexer)?),
            }
        }

        util::exp_cur_next_tok(lexer, Token::Special(";"))?;

        Ok(Block { lines })
    }
}
