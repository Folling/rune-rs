use crate::ast::evaluative::Expr;
use crate::ast::Node;
use crate::transpiler::ParseError;
use crate::Lexer;

#[derive(Debug)]
pub struct TupleLit<'a> {
    values: Vec<Expr<'a>>,
}

impl<'a> Node<'a> for TupleLit<'a> {
    fn generate(&self, content: &mut String) {
        todo!()
    }

    fn valid(&self) -> bool {
        todo!()
    }
}

impl<'a> TupleLit<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        todo!()
    }
}
