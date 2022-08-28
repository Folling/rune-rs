use crate::ast::evaluative::expression;
use crate::ast::typical::Type;

pub struct GenericType<'a> {
    r#type: Box<Type<'a>>,
}

pub enum Generic<'a> {
    Type(GenericType<'a>),
    Expression(expression::CompileTime),
}
