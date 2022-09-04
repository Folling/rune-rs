use crate::ast::evaluative::Block;
use crate::ast::structural::*;
use crate::ast::{Ident, Node};
use crate::transpiler::ParseError;
use crate::{Lexer, Token};

#[derive(Debug)]
pub enum TopLevelExpression<'a> {
    StructDecl(StructDecl<'a>),
    StructImpl(StructImpl<'a>),
    FuncDecl(FuncDecl<'a>),
    TraitDecl(TraitDecl<'a>),
    UseDecl(UseDecl<'a>),
}

impl<'a> Node<'a> for TopLevelExpression<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        match self {
            TopLevelExpression::StructDecl(v) => v.valid(),
            TopLevelExpression::StructImpl(v) => v.valid(),
            TopLevelExpression::FuncDecl(v) => v.valid(),
            TopLevelExpression::TraitDecl(v) => v.valid(),
            TopLevelExpression::UseDecl(v) => v.valid(),
        }
    }

    fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        match lexer.cur_next() {
            None => Err(ParseError::PrematureEOF),
            Some(Token::Textual("fn")) => Ok(TopLevelExpression::FuncDecl(FuncDecl::parse(lexer)?)),
            Some(Token::Textual("struct")) => Ok(TopLevelExpression::StructDecl(StructDecl::parse(lexer)?)),
            Some(Token::Textual("trait")) => Ok(TopLevelExpression::TraitDecl(TraitDecl::parse(lexer)?)),
            Some(Token::Textual("impl")) => Ok(TopLevelExpression::StructImpl(StructImpl::parse(lexer)?)),
            Some(Token::Textual("use")) => Ok(TopLevelExpression::UseDecl(UseDecl::parse(lexer)?)),
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
