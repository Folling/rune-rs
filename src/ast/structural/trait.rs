use crate::ast::structural::{Func, FuncProto};
use crate::ast::Ident;

pub enum TraitFunction<'a> {
    Unspecified(FuncProto<'a>),
    Specified(Func<'a>),
}

pub struct Trait<'a> {
    ident: Ident<'a>,
    funcs: Vec<TraitFunction<'a>>,
}
