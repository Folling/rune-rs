use crate::ast::structural::Func;
use crate::ast::Ident;

pub struct StructImpl<'a> {
    ident: Ident<'a>,
    funcs: Vec<Func<'a>>,
}
