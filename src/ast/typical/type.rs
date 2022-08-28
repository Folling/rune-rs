use crate::ast::Ident;

pub struct Type<'a> {
    r#mut: bool,
    ident: Ident<'a>,
    // maximum of 16 generics, avoids heap allocation
    generics: [&'a str; 16],
    // pointer levels are stored in the
    pointer_levels: u64,
}
