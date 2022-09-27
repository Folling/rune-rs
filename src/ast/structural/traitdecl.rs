use crate::ast::structural::{FuncDecl, FuncProto};
use crate::ast::{Ident, Node};
use crate::transpiler::ParseError;
use crate::Lexer;

#[derive(Debug)]
pub enum TraitFunction<'a> {
    Unspecified(FuncProto<'a>),
    Specified(FuncDecl<'a>),
}

#[derive(Debug)]
pub struct TraitDecl<'a> {
    ident: Ident<'a>,
    funcs: Vec<TraitFunction<'a>>,
}

impl<'a> Node<'a> for TraitDecl<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        self.ident.valid() && self.funcs.iter().all(Node::valid)
    }
}

impl<'a> TraitDecl<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl<'a> Node<'a> for TraitFunction<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        match self {
            TraitFunction::Unspecified(v) => v.valid(),
            TraitFunction::Specified(v) => v.valid(),
        }
    }
}
