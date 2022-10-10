use crate::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ParseErrorInfo<'a> {
    pub line: usize,
    pub column: usize,
    pub err: ParseErr<'a>,
}

impl<'a> Display for ParseErrorInfo<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} | {}", self.line, self.column, self.err)
    }
}

impl<'a> std::error::Error for ParseErrorInfo<'a> {}

#[derive(Debug)]
pub enum LiteralType {
    Char,
    String,
    Numeric,
    Tuple,
    Lambda,
}

#[derive(Debug)]
pub enum ExpectedToken {
    Textual { regex: &'static str },
    Special { regex: &'static str },
}

#[derive(Debug)]
pub enum ParseErr<'a> {
    PrematureEOF,
    InvalidToken {
        got: Token<'a>,
        expected: Vec<ExpectedToken>,
    },
    InvalidLiteral {
        r#type: LiteralType,
        got: &'a str,
        expected: &'static str,
    },
}

impl<'a> Display for ParseErr<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> std::error::Error for ParseErr<'a> {}
