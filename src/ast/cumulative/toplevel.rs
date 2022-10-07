use crate::ast::structural::*;
use crate::ast::Node;
use crate::transpiler::{ExpectedToken, ParseErr};
use crate::{Lexer, Token};

#[derive(Debug)]
pub enum TopLevelExpr<'a> {
    StructDecl(StructDecl<'a>),
    StructImpl(StructImpl<'a>),
    FuncDecl(FuncDecl<'a>),
    TraitDecl(TraitDecl<'a>),
    UseDecl(UseDecl<'a>),
}

impl<'a> Node<'a> for TopLevelExpr<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        match self {
            TopLevelExpr::StructDecl(val) => val.valid(),
            TopLevelExpr::StructImpl(val) => val.valid(),
            TopLevelExpr::FuncDecl(val) => val.valid(),
            TopLevelExpr::TraitDecl(val) => val.valid(),
            TopLevelExpr::UseDecl(val) => val.valid(),
        }
    }
}

impl<'a> TopLevelExpr<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseErr<'a>>
    where
        Self: Sized,
    {
        match lexer.cur_next() {
            None => Err(ParseErr::PrematureEOF),
            Some((Token::Textual("fn"), _)) => Ok(TopLevelExpr::FuncDecl(FuncDecl::parse(lexer)?)),
            Some((Token::Textual("struct"), _)) => Ok(TopLevelExpr::StructDecl(StructDecl::parse(lexer)?)),
            Some((Token::Textual("trait"), _)) => Ok(TopLevelExpr::TraitDecl(TraitDecl::parse(lexer)?)),
            Some((Token::Textual("impl"), _)) => Ok(TopLevelExpr::StructImpl(StructImpl::parse(lexer)?)),
            Some((Token::Textual("use"), _)) => Ok(TopLevelExpr::UseDecl(UseDecl::parse(lexer)?)),
            Some((val, _)) => Err(ParseErr::InvalidToken {
                got: val,
                expected: vec![
                    ExpectedToken::Textual { regex: "fn" },
                    ExpectedToken::Textual { regex: "struct" },
                    ExpectedToken::Textual { regex: "trait" },
                    ExpectedToken::Textual { regex: "impl" },
                    ExpectedToken::Textual { regex: "use" },
                ],
            }),
        }
    }
}
