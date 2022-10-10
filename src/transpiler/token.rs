use crate::transpiler::ExpectedToken;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenLoc(pub(in crate::transpiler) usize, pub(in crate::transpiler) usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token<'a> {
    Textual(&'a str),
    Special(&'a str),
}
