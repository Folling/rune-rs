use crate::transpiler::ParseError;
use crate::{Lexer, Token};

pub fn exp_cur_next_tok<'a>(lexer: &mut Lexer<'a>, token: Token<'static>) -> Result<(), ParseError<'a>> {
    match lexer.cur_next() {
        None => Err(ParseError::PrematureEOF),
        Some(v) if v == token => Ok(()),
        Some(v) => Err(ParseError::InvalidToken {
            got: v,
            expected: vec![token],
        }),
    }
}

pub fn exp_next_cur_tok<'a>(lexer: &mut Lexer<'a>, token: Token<'static>) -> Result<(), ParseError<'a>> {
    match lexer.next_cur() {
        None => Err(ParseError::PrematureEOF),
        Some(v) if v == token => Ok(()),
        Some(v) => Err(ParseError::InvalidToken {
            got: v,
            expected: vec![token],
        }),
    }
}

pub fn exp_next_cur_toks<'a>(lexer: &mut Lexer<'a>, tokens: &[Token<'static>]) -> Result<(), ParseError<'a>> {
    match lexer.next_cur() {
        None => Err(ParseError::PrematureEOF),
        Some(v) if tokens.contains(&v) => Ok(()),
        Some(v) => Err(ParseError::InvalidToken {
            got: v,
            expected: tokens.to_vec(),
        }),
    }
}
