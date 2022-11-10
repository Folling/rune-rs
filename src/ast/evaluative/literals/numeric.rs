use crate::ast::Node;
use crate::transpiler::{ExpectedToken, LiteralType, ParseErr, TokenLoc};
use crate::{Lexer, Token};

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
        return match lexer.cur_next() {
            None => Err(ParseErr::PrematureEOF),
            Some((Token::Special("."), _)) => match lexer.cur_next() {
                None => Err(ParseErr::PrematureEOF),
                Some((Token::Textual(val), loc)) if matches!(val.chars().next(), Some(c) if c.is_ascii_digit()) => {
                    let value = unsafe { lexer.get_from_to(from_loc, loc) };

                    match lexer.cur_next() {
                        None => return Err(ParseErr::PrematureEOF),
                        Some((Token::Special("_"), _)) => match lexer.cur_next() {
                            None => Err(ParseErr::PrematureEOF),
                            Some((Token::Textual(val), _)) if ["f32", "f64"].contains(&val) => Ok(Self { value, r#type: val }),
                            Some((val, loc2)) => Err(ParseErr::InvalidLiteral {
                                r#type: LiteralType::Numeric,
                                got: unsafe { lexer.get_from_to(loc, loc2) },
                                expected: "(f32|f64)",
                            }),
                        },
                        Some(_) => Err(ParseErr::InvalidLiteral {
                            r#type: LiteralType::Numeric,
                            got: unsafe { lexer.get_from_to(from_loc, loc) },
                            expected: "_",
                        }),
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
            Some((Token::Special("_"), _)) => match lexer.cur_next() {
                None => Err(ParseErr::PrematureEOF),
                Some((Token::Textual(val), _)) if ["i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64"].contains(&val) => Ok(Self {
                    value: from_value,
                    r#type: val,
                }),
                Some((val, _)) => Err(ParseErr::InvalidToken {
                    got: val,
                    expected: vec![ExpectedToken::Textual { regex: "[ui](8|16|32|64)" }],
                }),
            },
            Some((val, _)) => Err(ParseErr::InvalidToken {
                got: val,
                expected: vec![ExpectedToken::Textual { regex: "[\\._]" }],
            }),
        };
    }
}
