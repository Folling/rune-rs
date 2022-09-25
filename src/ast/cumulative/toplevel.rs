use crate::ast::evaluative::Block;
use crate::ast::structural::*;
use crate::ast::{Ident, Node};
use crate::transpiler::ParseError;
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
            TopLevelExpr::StructDecl(v) => v.valid(),
            TopLevelExpr::StructImpl(v) => v.valid(),
            TopLevelExpr::FuncDecl(v) => v.valid(),
            TopLevelExpr::TraitDecl(v) => v.valid(),
            TopLevelExpr::UseDecl(v) => v.valid(),
        }
    }
}

impl<'a> TopLevelExpr<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        match lexer.cur_next() {
            None => Err(ParseError::PrematureEOF),
            Some(Token::Textual("fn")) => Ok(TopLevelExpr::FuncDecl(FuncDecl::parse(lexer)?)),
            Some(Token::Textual("struct")) => Ok(TopLevelExpr::StructDecl(StructDecl::parse(lexer)?)),
            Some(Token::Textual("trait")) => Ok(TopLevelExpr::TraitDecl(TraitDecl::parse(lexer)?)),
            Some(Token::Textual("impl")) => Ok(TopLevelExpr::StructImpl(StructImpl::parse(lexer)?)),
            Some(Token::Textual("use")) => Ok(TopLevelExpr::UseDecl(UseDecl::parse(lexer)?)),
            Some(v) => Err(ParseError::InvalidToken {
                got: v,
                expected: vec![
                    Token::Textual("fn"),
                    Token::Textual("struct"),
                    Token::Textual("trait"),
                    Token::Textual("impl"),
                    Token::Textual("use"),
                ],
            }),
        }
    }
}
