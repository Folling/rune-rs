use crate::ast::typical::Type;
use crate::ast::{Ident, Node};
use crate::transpiler::ParseError;
use crate::Lexer;

#[derive(Debug)]
pub struct StructField<'a> {
    mutable: bool,
    ident: Ident<'a>,
    r#type: Type<'a>,
}

#[derive(Debug)]
pub struct StructDecl<'a> {
    ident: Ident<'a>,
    fields: Vec<(Ident<'a>, Type<'a>)>,
}

impl<'a> Node<'a> for StructDecl<'a> {
    fn generate(&self, content: &mut String) {
        todo!()
    }

    fn valid(&self) -> bool {
        self.ident.valid() && self.fields.iter().all(|(i, t)| i.valid() && t.valid())
    }
}

impl<'a> StructDecl {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        todo!()
    }
}
