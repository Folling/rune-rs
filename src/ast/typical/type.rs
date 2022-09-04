use crate::ast::evaluative::Expr;
use crate::ast::{Ident, Node};
use crate::transpiler::ParseError;
use crate::Lexer;

#[derive(Debug)]
pub enum Generic<'a> {
    Type(Box<Type<'a>>),
    Expr(Box<Expr<'a>>),
}

#[derive(Debug)]
pub struct PointerLevel {
    r#mut: bool,
}

#[derive(Debug)]
pub struct Type<'a> {
    r#mut: bool,
    ident: Ident<'a>,
    generics: Vec<Generic<'a>>,
    pointer_levels: Vec<PointerLevel>,
}

impl<'a> Node<'a> for Type<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        true
    }

    fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        Ok(Type {
            r#mut: false,
            ident: Ident::parse(lexer)?,
            generics: vec![],
            pointer_levels: vec![],
        })
    }
}
