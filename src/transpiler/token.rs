use crate::transpiler::ExpectedToken;

#[derive(copy, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenLoc(pub(in crate::transpiler) usize, pub(in crate::transpiler) usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token<'a> {
    Textual(&'a str),
    Special(&'a str),
}

impl<'a> Token<'a> {
    pub fn to_expected(&self) -> ExpectedToken {
        match self {
            Token::Textual(val) => ExpectedToken::Textual { regex: val },
            Token::Special(val) => ExpectedToken::Special { regex: val },
        }
    }
}
