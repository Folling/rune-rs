use crate::ast::structural::FuncDecl;
use crate::ast::{Ident, Node};
use crate::transpiler::ParseError;
use crate::Lexer;

#[derive(Debug)]
pub struct StructImpl<'a> {
    ident: Ident<'a>,
    funcs: Vec<FuncDecl<'a>>,
}

impl<'a> Node<'a> for StructImpl<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        self.ident.valid() && self.funcs.iter().all(Node::valid)
    }

    fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        todo!()
    }
}
