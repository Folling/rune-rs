use crate::Token;

#[derive(Debug)]
pub struct ParseErrorInfo<'a> {
    pub line: usize,
    pub column: usize,
    pub err: ParseError<'a>,
}

#[derive(Debug)]
pub enum ParseError<'a> {
    PrematureEOF,
    InvalidToken { got: Token<'a>, expected: Vec<Token<'a>> },
}
