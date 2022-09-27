use crate::transpiler::{ExpectedToken, ParseError};
use crate::{Lexer, Token};

fn expr_sp_token<'a>(token: Option<Token<'a>>, exp: &'static str) -> Result<(), ParseError<'a>> {
    match token {
        None => Err(ParseError::PrematureEOF),
        Some(Token::Special { value, .. }) if value == exp => Ok(()),
        Some(v) => Err(ParseError::InvalidToken {
            got: v,
            expected: vec![ExpectedToken::Special { regex: exp }],
        }),
    }
}

pub fn exp_cur_next_sp_tok<'a>(lexer: &mut Lexer<'a>, exp: &'static str) -> Result<(), ParseError<'a>> {
    expr_sp_token(lexer.cur_next(), exp)
}

pub fn exp_next_cur_sp_tok<'a>(lexer: &mut Lexer<'a>, exp: &'static str) -> Result<(), ParseError<'a>> {
    expr_sp_token(lexer.next_cur(), exp)
}

pub fn exp_next_cur_sp_toks<'a>(lexer: &mut Lexer<'a>, exp: &[&'static str]) -> Result<(), ParseError<'a>> {
    match lexer.next_cur() {
        None => Err(ParseError::PrematureEOF),
        Some(Token::Special { value, .. }) if exp.contains(&value) => Ok(()),
        Some(v) => Err(ParseError::InvalidToken {
            got: v,
            expected: tokens.iter().map(|v| ExpectedToken::Special { regex: v }).collect::<Vec<_>>(),
        }),
    }
}
