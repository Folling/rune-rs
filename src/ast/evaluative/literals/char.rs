use crate::ast::Node;
use crate::transpiler::{util, ExpectedToken, ParseErr};
use crate::{Lexer, Token};

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

                u32::from_str_radix(str, 16).ok().and_then(char::from_u32).is_some()
            }
            (Some((_, '\\')), Some(_)) => true,
            _ => false,
        }
    }
}

impl<'a> CharLit<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseErr<'a>>
    where
        Self: Sized,
    {
        let value = match lexer.cur_next() {
            None => return Err(ParseErr::PrematureEOF),
            Some((Token::Textual(val), _)) => val,
            Some((val, _)) => {
                return Err(ParseErr::InvalidToken {
                    got: val,
                    expected: vec![ExpectedToken::Textual {
                        regex: "([\\p{L}\\p{M}|\\\\u([0x0-0xD77]|[0xE000-0x10FFFF])",
                    }],
                })
            }
        };

        util::exp_cur_next_sp_tok(lexer, "'")?;

        Ok(CharLit { value })
    }
}
