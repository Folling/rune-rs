use crate::ast::evaluative::Block;
use crate::ast::typical::Type;
use crate::ast::Ident;

pub struct LambdaArg<'a> {
    ident: Ident<'a>,
    type_hint: Option<Type<'a>>,
}

pub struct Lambda<'a> {
    args: Vec<LambdaArg<'a>>,
    body: Block,
}
