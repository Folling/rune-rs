use crate::ast::Node;
use crate::transpiler::{ExpectedToken, LiteralType, ParseErr, TokenLoc};
use crate::{Lexer, Token};
use std::str::pattern::Pattern;

#[derive(Debug)]
pub struct NumLit<'a> {
    pub value: &'a str,
    pub r#type: &'a str,
}

impl<'a> Node<'a> for NumLit<'a> {
    fn generate(&self, content: &mut String) {
        todo!()
    }

    fn valid(&self) -> bool {
        todo!()
    }
}

impl<'a> NumLit<'a> {
    pub fn parse(lexer: &mut Lexer<'a>, from_value: &'a str, from_loc: TokenLoc) -> Result<Self, ParseErr<'a>> {
        let ret = match lexer.cur_next() {
            None => return Err(ParseErr::PrematureEOF),
            Some((Token::Special("."), _)) => match lexer.cur_next() {
                None => return Err(ParseErr::PrematureEOF),
                Some((Token::Textual(val), loc)) if matches!(val.chars().next(), Some(c) if c.is_ascii_digit()) => {
                    let value = unsafe { lexer.get_from_to(from_loc, loc) };

                    match lexer.cur_next() {
                        None => return Err(ParseErr::PrematureEOF),
                        Some((Token::Special("_"), loc)) => {}
                        Some(_) => {
                            return Err(ParseErr::InvalidLiteral {
                                r#type: LiteralType::Numeric,
                                got: unsafe { lexer.get_from_to(from_loc, loc) },
                                expected: "_",
                            })
                        }
                    }
                }
                Some((_, loc)) => {
                    return Err(ParseErr::InvalidLiteral {
                        r#type: LiteralType::Numeric,
                        got: unsafe { lexer.get_from_to(from_loc, loc) },
                        expected: "\\d+",
                    })
                }
            },
            Some((Token::Special("_"), _)) => {
                return match lexer.cur_next() {
                    None => Err(ParseErr::PrematureEOF),
                    Some((Token::Textual("i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64"), _)) => Ok(Self {
                        value: from_value,
                        r#type: val,
                    }),
                    Some((val, _)) => Err(ParseErr::InvalidToken {
                        got: val,
                        expected: vec![ExpectedToken::Textual { regex: "[ui](8|16|32|64)" }],
                    }),
                }
            }
            Some((val, _)) => {
                return Err(ParseErr::InvalidToken {
                    got: val,
                    expected: vec![ExpectedToken::Textual { regex: "[\\._]" }],
                })
            }
        };
    }
}
