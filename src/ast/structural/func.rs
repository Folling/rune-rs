use crate::ast::evaluative::Block;
use crate::ast::typical::Type;
use crate::ast::Ident;

pub struct FuncArg<'a> {
    ident: Ident<'a>,
    r#type: Type<'a>,
}

pub struct FuncRet<'a> {
    ident: Option<Ident<'a>>,
    r#type: Type<'a>,
}

pub struct FuncProto<'a> {
    ident: Ident<'a>,
    args: Vec<FuncArg<'a>>,
    ret: Vec<FuncRet<'a>>,
}

pub struct Func<'a> {
    proto: FuncProto<'a>,
    body: Block,
}
