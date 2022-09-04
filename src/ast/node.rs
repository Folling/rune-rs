use crate::transpiler::ParseError;
use crate::Lexer;
use std::error::Error;
use std::fmt::Debug;

pub trait Node<'a>: Debug {
    fn generate(&self, content: &mut String);
    fn valid(&self) -> bool;
    fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized;
}
