use crate::transpiler::{ExpectedToken, ParseErr, TokenLoc};
use crate::{Lexer, Token};

fn expr_sp_token<'a>(token: Option<(Token<'a>, TokenLoc)>, exp: &'static str) -> Result<(), ParseErr<'a>> {
    match token {
        None => Err(ParseErr::PrematureEOF),
        Some((Token::Special(val), _)) if val == exp => Ok(()),
        Some((val, _)) => Err(ParseErr::InvalidToken {
            got: val,
            expected: vec![ExpectedToken::Special { regex: exp }],
        }),
    }
}

pub fn exp_cur_next_sp_tok<'a>(lexer: &mut Lexer<'a>, exp: &'static str) -> Result<(), ParseErr<'a>> {
    expr_sp_token(lexer.cur_next(), exp)
}

pub fn exp_next_cur_sp_tok<'a>(lexer: &mut Lexer<'a>, exp: &'static str) -> Result<(), ParseErr<'a>> {
    expr_sp_token(lexer.next_cur(), exp)
}

pub fn exp_next_cur_sp_toks<'a>(lexer: &mut Lexer<'a>, exp: &[&'static str]) -> Result<(), ParseErr<'a>> {
    match lexer.next_cur() {
        None => Err(ParseErr::PrematureEOF),
        Some((Token::Special(val), _)) if exp.contains(&val) => Ok(()),
        Some((val, _)) => Err(ParseErr::InvalidToken {
            got: val,
            expected: exp.iter().map(|val| ExpectedToken::Special { regex: val }).collect::<Vec<_>>(),
        }),
    }
}
