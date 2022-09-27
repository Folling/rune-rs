use crate::ast::Node;
use crate::transpiler::{ExpectedToken, LiteralType, ParseError};
use crate::{Lexer, Token};

#[derive(Debug)]
pub struct NumericLit<'a> {
    value: &'a str,
    r#type: &'a str,
}

impl<'a> Node<'a> for NumericLit {
    fn generate(&self, content: &mut String) {
        todo!()
    }

    fn valid(&self) -> bool {
        todo!()
    }
}

impl<'a> NumericLit<'a> {
    pub fn parse(lexer: &mut Lexer<'a>, from_value: &'a str, from_idx: usize) -> Result<Self, ParseError> {
        let ret = match lexer.cur() {
            None => return Err(ParseError::PrematureEOF),
            Some(Token::Special { value: ".", .. }) => match lexer.peek() {
                None => return Err(ParseError::PrematureEOF),
                Some(Token::Textual { value, idx }) if matches!(value.chars().next(), Some(c) if c.is_ascii_digit()) => {}
                _ => {
                    return Err(ParseError::InvalidLiteral {
                        r#type: LiteralType::Numeric,
                        got: unsafe { lexer.get_from_to_unchecked(from_idx, idx) },
                        expected: "\\d+(([ui](8|16|32|64))|(\\.\\d+))",
                    })
                }
            },
            Some(Token::Special { value: "_", .. }) => {}
            Some(Token::Textual { value: "i8", .. }) => {}
            Some(Token::Textual { value: "i16", .. }) => {}
            Some(Token::Textual { value: "i32", .. }) => {}
            Some(Token::Textual { value: "i64", .. }) => {}
            Some(Token::Textual { value: "u8", .. }) => {}
            Some(Token::Textual { value: "u16", .. }) => {}
            Some(Token::Textual { value: "u32", .. }) => {}
            Some(Token::Textual { value: "u64", .. }) => {}
            Some(v) => {
                return Err(ParseError::InvalidToken {
                    got: v,
                    expected: vec![ExpectedToken::Textual {
                        regex: "\\d+(([ui](8|16|32|64))|(\\.\\d+))",
                    }],
                })
            }
        };

        lexer.next_cur();

        Ok(ret)
    }
}
