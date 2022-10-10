use crate::ast::evaluative::Expr;
use crate::ast::Node;
use crate::transpiler::{util, ExpectedToken, ParseErr, TokenLoc};
use crate::{Lexer, Token};

#[derive(Debug)]
pub struct TupleLit<'a> {
    values: Vec<Expr<'a>>,
}

impl<'a> Node<'a> for TupleLit<'a> {
    fn generate(&self, content: &mut String) {
        todo!()
    }

    fn valid(&self) -> bool {
        todo!()
    }
}

impl<'a> TupleLit<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseErr<'a>>
    where
        Self: Sized,
    {
        let mut values = Vec::new();

        if let Some((Token::Special("]"), _)) = lexer.cur() {
            return Ok(TupleLit { values });
        }

        loop {
            values.push(Expr::parse(lexer)?);

            match lexer.cur_next() {
                None => return Err(ParseErr::PrematureEOF),
                Some((Token::Special(","), _)) => continue,
                Some((Token::Special("]"), _)) => return Ok(TupleLit { values }),
                Some((val, _)) => {
                    return Err(ParseErr::InvalidToken {
                        got: val,
                        expected: vec![ExpectedToken::Special { regex: "," }],
                    })
                }
            }
        }
    }
}
