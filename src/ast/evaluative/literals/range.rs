use crate::ast::evaluative::Expr;
use crate::ast::Node;
use crate::transpiler::ParseErr;
use crate::util::notneither::NotNeither;
use crate::Lexer;

#[derive(Debug)]
pub enum RangeOpenness {
    Closed,
    HalfOpenLower,
    HalfOpenUpper,
    Open,
}

#[derive(Debug)]
pub struct RangeLit<'a> {
    pub from: Box<Expr<'a>>,
    pub to: Box<Expr<'a>>,
    pub openness: RangeOpenness,
}

impl<'a> Node<'a> for RangeLit<'a> {
    fn generate(&self, content: &mut String) {
        todo!()
    }

    fn valid(&self) -> bool {
        todo!()
    }
}

impl<'a> RangeLit<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseErr<'a>>
    where
        Self: Sized,
    {
        todo!()
    }
}
