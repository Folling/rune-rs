use crate::ast::evaluative::Expr;
use crate::ast::typical::Type;
use crate::ast::Node;
use crate::transpiler::ParseErr;
use crate::Lexer;

#[derive(Debug)]
pub struct GenericType<'a> {
    r#type: Box<Type<'a>>,
}

#[derive(Debug)]
pub enum Generic<'a> {
    Type(GenericType<'a>),
    Expression(Expr<'a>),
}

impl<'a> Node<'a> for Generic<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        match self {
            Generic::Type(val) => val.valid(),
            Generic::Expression(val) => val.valid(),
        }
    }
}

impl<'a> Generic<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseErr<'a>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl<'a> Node<'a> for GenericType<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        self.r#type.valid()
    }
}

impl<'a> GenericType<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseErr<'a>>
    where
        Self: Sized,
    {
        todo!()
    }
}
