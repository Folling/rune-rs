use crate::ast::{Line, Node};
use crate::transpiler::{util, ParseErr};
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
    pub fn parse(lexer: &mut Lexer<'a>, semicolon: bool) -> Result<Self, ParseErr<'a>>
    where
        Self: Sized,
    {
        util::exp_cur_next_sp_tok(lexer, "{")?;

        let mut lines = vec![];

        if let Some((Token::Special("}"), _)) = lexer.cur() {
            lexer.skip();
            return Ok(Block { lines });
        }

        loop {
            lines.push(Line::parse(lexer)?);

            println!("{:?}", lexer.cur());

            match lexer.cur() {
                None => return Err(ParseErr::PrematureEOF),
                Some((Token::Special("}"), _)) => {
                    lexer.skip();
                    break;
                }
                _ => continue,
            }
        }

        if semicolon {
            util::exp_cur_next_sp_tok(lexer, ";")?;
        }

        Ok(Block { lines })
    }
}
