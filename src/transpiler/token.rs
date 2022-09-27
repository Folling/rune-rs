use crate::transpiler::ExpectedToken;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token<'a> {
    Textual { loc: (usize, usize), value: &'a str },
    Special { loc: (usize, usize), value: &'a str },
}

impl<'a> Token {
    pub fn to_expected(&self) -> ExpectedToken {
        match self {
            Token::Textual { value, .. } => ExpectedToken::Textual { regex: value },
            Token::Special { value, .. } => ExpectedToken::Special { regex: value },
        }
    }
}
