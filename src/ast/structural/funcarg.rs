use crate::ast::typical::Type;
use crate::ast::{Ident, Node};
use crate::transpiler::{util, ParseError};
use crate::{Lexer, Token};

#[derive(Debug)]
pub struct FuncArg<'a> {
    ident: Ident<'a>,
    r#type: Type<'a>,
}

impl<'a> Node<'a> for FuncArg<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        self.ident.valid() && self.r#type.valid()
    }
}

impl<'a> FuncArg<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        let ident = Ident::parse(lexer)?;

        util::exp_cur_next_sp_tok(lexer, ":")?;

        let r#type = Type::parse(lexer)?;

        Ok(FuncArg { ident, r#type })
    }
}
