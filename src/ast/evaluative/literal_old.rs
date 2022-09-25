// use crate::ast::evaluative::Block;
// use crate::ast::evaluative::Expr;
// use crate::ast::typical::Type;
// use crate::ast::{Ident, Node};
// use crate::transpiler::{util, ParseError};
// use crate::util::notneither::NotNeither;
// use crate::{Lexer, Token};
// use std::io::Error;
// use std::thread::park;
//
// #[derive(Debug)]
// pub enum Literal<'a> {
//     Array {
//         items: Vec<Box<Expr<'a>>>,
//     },
//     Floating {
//         value: FloatingLiteral,
//     },
//     Integer {
//         value: IntegerLiteral,
//     },
//     Lambda {
//         args: Vec<LambdaArg<'a>>,
//         body: Box<Block<'a>>,
//     },
//     Range {
//         from_to: NotNeither<Box<Expr<'a>>>,
//         openness: RangeOpenness,
//     },
//     Char {
//         value: char,
//     },
//     String {
//         value: &'a str,
//     },
//     Tuple {
//         items: Vec<Expr<'a>>,
//     },
// }
//
// #[derive(Debug)]
// pub enum FloatingLiteral {
//     F32(f32),
//     F64(f64),
// }
//
// #[derive(Debug)]
// pub enum IntegerLiteral {
//     I8(i8),
//     I16(i16),
//     I32(i32),
//     I64(i64),
//     U8(u8),
//     U16(u16),
//     U32(u32),
//     U64(u64),
// }
//
// #[derive(Debug)]
// pub struct LambdaArg<'a> {
//     ident: Ident<'a>,
//     type_hint: Option<Type<'a>>,
// }
//
// #[derive(Debug)]
// pub enum RangeOpenness {
//     Closed,
//     HalfOpenLower,
//     HalfOpenUpper,
//     Open,
// }
//
// impl<'a> Node<'a> for Literal<'a> {
//     fn generate(&self, content: &mut String) {}
//
//     fn valid(&self) -> bool {
//         true
//     }
//
//     pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
//     where
//         Self: Sized,
//     {
//         let start_idx = lexer.token_idx();
//
//         match lexer.cur_next() {
//             None => return Err(ParseError::PrematureEOF),
//             Some(Token::Special("'")) => {
//                 let ret = match lexer.cur_next() {
//                     None => return Err(ParseError::PrematureEOF),
//                     Some(Token::Textual(v)) if v.len() == 1 => Literal::Char {
//                         value: unsafe { v.chars().next().unwrap_unchecked() },
//                     },
//                     Some(v) => {
//                         return Err(ParseError::InvalidToken {
//                             got: v,
//                             expected: vec![Token::Textual("single char")],
//                         })
//                     }
//                 };
//
//                 util::exp_cur_next_tok(lexer, Token::Special("'"))?;
//
//                 Ok(ret)
//             }
//             Some(Token::Special("\"")) => {
//                 let ret = match lexer.cur_next() {
//                     None => return Err(ParseError::PrematureEOF),
//                     Some(Token::Textual(v)) => Literal::String { value: v },
//                     Some(v) => {
//                         return Err(ParseError::InvalidToken {
//                             got: v,
//                             expected: vec![Token::Textual("any text")],
//                         })
//                     }
//                 };
//
//                 util::exp_cur_next_tok(lexer, Token::Special("\""))?;
//
//                 Ok(ret)
//             }
//             Some(Token::Special("[")) => {
//                 let ret = match lexer.cur() {
//                     None => Err(ParseError::PrematureEOF),
//                     Some(Token::Special("..")) => {
//                         lexer.next_cur();
//
//                         let ret = match lexer.cur() {
//                             None => Err(ParseError::PrematureEOF),
//                             Some(Token::Special("<")) => {
//                                 lexer.next_cur();
//                                 Ok(Literal::Range {
//                                     from_to: NotNeither::Right(Box::new(Expr::parse(lexer)?)),
//                                     openness: RangeOpenness::HalfOpenUpper,
//                                 })
//                             }
//                             Some(_) => Ok(Literal::Range {
//                                 from_to: NotNeither::Right(Box::new(Expr::parse(lexer)?)),
//                                 openness: RangeOpenness::Closed,
//                             }),
//                         };
//
//                         util::exp_cur_next_tok(lexer, Token::Special("]"))?;
//
//                         ret
//                     }
//                     Some(_) => {
//                         let expr = Box::new(Expr::parse(lexer)?);
//
//                         match lexer.cur_next() {
//                             None => Err(ParseError::PrematureEOF),
//                             Some(Token::Special(",")) => {
//                                 let mut items = vec![expr];
//
//                                 loop {
//                                     items.push(Box::new(Expr::parse(lexer)?));
//
//                                     match lexer.cur_next() {
//                                         None => return Err(ParseError::PrematureEOF),
//                                         Some(Token::Special(",")) => continue,
//                                         Some(Token::Special("]")) => break,
//                                         Some(v) => {
//                                             return Err(ParseError::InvalidToken {
//                                                 got: v,
//                                                 expected: vec![Token::Special(","), Token::Special("]")],
//                                             })
//                                         }
//                                     }
//                                 }
//
//                                 Ok(Literal::Array { items })
//                             }
//                             Some(Token::Special(">")) => {
//                                 util::exp_cur_next_tok(lexer, Token::Special(".."))?;
//
//                                 Literal::parse_range_upper(lexer, expr, true)
//                             }
//                             Some(Token::Special("..")) => Literal::parse_range_upper(lexer, expr, false),
//                             Some(v) => Err(ParseError::InvalidToken {
//                                 got: v,
//                                 expected: vec![Token::Special(","), Token::Special("..")],
//                             }),
//                         }
//                     }
//                 };
//
//                 ret
//             }
//             Some(Token::Special("|")) => {
//                 todo!("lambdas aren't implemented yet")
//             }
//             Some(Token::Textual(v)) if matches!(v.chars().next(), Some(c) if c.is_ascii_digit()) => {
//                 return match lexer.cur() {
//                     None => Err(ParseError::PrematureEOF),
//                     Some(Token::Special(".")) => {
//                         lexer.next_cur();
//
//                         match lexer.cur_next() {
//                             None => Err(ParseError::PrematureEOF),
//                             Some(Token::Textual(_)) => {
//                                 let str = unsafe { lexer.get_from_to_unchecked(start_idx, lexer.token_idx()) };
//
//                                 util::exp_cur_next_tok(lexer, Token::Special("_"))?;
//
//                                 match lexer.cur_next() {
//                                     None => Err(ParseError::PrematureEOF),
//                                     Some(Token::Textual("f32")) => Ok(Literal::Floating {
//                                         value: FloatingLiteral::F32(str.parse::<f32>().map_err(|_| ParseError::InvalidLiteral {
//                                             got: str,
//                                             expected: "valid 32 bit float",
//                                         })?),
//                                     }),
//                                     Some(Token::Textual("f64")) => Ok(Literal::Floating {
//                                         value: FloatingLiteral::F64(str.parse::<f64>().map_err(|_| ParseError::InvalidLiteral {
//                                             got: str,
//                                             expected: "valid 64 bit float",
//                                         })?),
//                                     }),
//                                     Some(v) => Err(ParseError::InvalidToken {
//                                         got: v,
//                                         expected: vec![Token::Textual("f32"), Token::Textual("f64")],
//                                     }),
//                                 }
//                             }
//                             Some(v) => Err(ParseError::InvalidToken {
//                                 got: v,
//                                 expected: vec![Token::Textual("a valid fraction")],
//                             }),
//                         }
//                     }
//                     Some(_) => {
//                         let mut chars = v.chars();
//                         let first = chars.next().ok_or_else(|| ParseError::PrematureEOF)?;
//
//                         let mut offset = 0;
//
//                         let base = if first == '0' {
//                             offset = 2;
//
//                             match chars.next() {
//                                 None => {
//                                     offset -= 1;
//                                     10
//                                 }
//                                 Some('b') => 2,
//                                 Some('o') => 8,
//                                 Some('x') => 16,
//                                 Some(_) => {
//                                     return Err(ParseError::InvalidLiteral {
//                                         got: v,
//                                         expected: "a base indicator('b', 'o', or 'x')",
//                                     })
//                                 }
//                             }
//                         } else {
//                             10
//                         };
//
//                         let str = &v[offset..];
//
//                         util::exp_cur_next_tok(lexer, Token::Special("_"))?;
//
//                         match lexer.cur_next() {
//                             None => Err(ParseError::PrematureEOF),
//                             Some(Token::Textual("i8")) => Ok(Literal::Integer {
//                                 value: IntegerLiteral::I8(i8::from_str_radix(str, base).map_err(|e| ParseError::InvalidLiteral {
//                                     got: v,
//                                     expected: "-128 to 127",
//                                 })?),
//                             }),
//                             Some(Token::Textual("i16")) => Ok(Literal::Integer {
//                                 value: IntegerLiteral::I16(i16::from_str_radix(str, base).map_err(|e| ParseError::InvalidLiteral {
//                                     got: v,
//                                     expected: "-32'768 to 32'767",
//                                 })?),
//                             }),
//                             Some(Token::Textual("i32")) => Ok(Literal::Integer {
//                                 value: IntegerLiteral::I32(i32::from_str_radix(str, base).map_err(|e| ParseError::InvalidLiteral {
//                                     got: v,
//                                     expected: "-2147483648 to 2147483647",
//                                 })?),
//                             }),
//                             Some(Token::Textual("i64")) => Ok(Literal::Integer {
//                                 value: IntegerLiteral::I64(i64::from_str_radix(str, base).map_err(|e| ParseError::InvalidLiteral {
//                                     got: v,
//                                     expected: "-9223372036854775808 to 9223372036854775807",
//                                 })?),
//                             }),
//                             Some(Token::Textual("u8")) => Ok(Literal::Integer {
//                                 value: IntegerLiteral::U8(u8::from_str_radix(str, base).map_err(|e| ParseError::InvalidLiteral {
//                                     got: v,
//                                     expected: "0 to 255",
//                                 })?),
//                             }),
//                             Some(Token::Textual("u16")) => Ok(Literal::Integer {
//                                 value: IntegerLiteral::U16(u16::from_str_radix(str, base).map_err(|e| ParseError::InvalidLiteral {
//                                     got: v,
//                                     expected: "0 to 65535",
//                                 })?),
//                             }),
//                             Some(Token::Textual("u32")) => Ok(Literal::Integer {
//                                 value: IntegerLiteral::U32(u32::from_str_radix(str, base).map_err(|e| ParseError::InvalidLiteral {
//                                     got: v,
//                                     expected: "0 to 4294967295",
//                                 })?),
//                             }),
//                             Some(Token::Textual("u64")) => Ok(Literal::Integer {
//                                 value: IntegerLiteral::U64(u64::from_str_radix(str, base).map_err(|e| ParseError::InvalidLiteral {
//                                     got: v,
//                                     expected: "0 to 18446744073709551615",
//                                 })?),
//                             }),
//                             Some(v) => Err(ParseError::InvalidToken {
//                                 got: v,
//                                 expected: vec![
//                                     Token::Textual("i8"),
//                                     Token::Textual("i16"),
//                                     Token::Textual("i32"),
//                                     Token::Textual("i64"),
//                                     Token::Textual("u8"),
//                                     Token::Textual("u16"),
//                                     Token::Textual("u32"),
//                                     Token::Textual("u64"),
//                                 ],
//                             }),
//                         }
//                     }
//                 };
//             }
//             Some(v) => {
//                 return Err(ParseError::InvalidToken {
//                     got: v,
//                     expected: vec![
//                         Token::Special("\""),
//                         Token::Special("'"),
//                         Token::Textual("0-9"),
//                         Token::Special("["),
//                         Token::Special("|"),
//                     ],
//                 })
//             }
//         }
//     }
// }
//
// impl<'a> Literal<'a> {
//     pub fn parse_range_upper(lexer: &mut Lexer<'a>, from: Box<Expr<'a>>, lower_open: bool) -> Result<Literal<'a>, ParseError<'a>> {
//         let ret = match lexer.cur() {
//             None => return Err(ParseError::PrematureEOF),
//             Some(Token::Special("<")) => {
//                 lexer.next_cur();
//
//                 let to = Box::new(Expr::parse(lexer)?);
//                 return Ok(Literal::Range {
//                     from_to: NotNeither::Both(from, to),
//                     openness: if lower_open {
//                         RangeOpenness::Open
//                     } else {
//                         RangeOpenness::HalfOpenUpper
//                     },
//                 });
//             }
//             Some(Token::Special("]")) => {
//                 return Ok(Literal::Range {
//                     from_to: NotNeither::Left(from),
//                     openness: if lower_open {
//                         RangeOpenness::HalfOpenLower
//                     } else {
//                         RangeOpenness::Closed
//                     },
//                 });
//             }
//             Some(_) => {
//                 let to = Box::new(Expr::parse(lexer)?);
//
//                 Ok(Literal::Range {
//                     from_to: NotNeither::Both(from, to),
//                     openness: if lower_open {
//                         RangeOpenness::HalfOpenLower
//                     } else {
//                         RangeOpenness::Closed
//                     },
//                 })
//             }
//         };
//
//         util::exp_cur_next_tok(lexer, Token::Special("]"))?;
//
//         ret
//     }
// }
