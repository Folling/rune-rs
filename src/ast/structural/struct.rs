use crate::ast::typical::Type;
use crate::ast::Ident;

pub struct StructField<'a> {
    mutable: bool,
    ident: Ident<'a>,
    r#type: Type<'a>,
}

pub struct Struct<'a> {
    ident: Ident<'a>,
    fields: Vec<(Ident<'a>, Type<'a>)>,
}
