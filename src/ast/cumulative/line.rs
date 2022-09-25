use crate::ast::evaluative::{Expr, VarDecl};
use crate::ast::{Ident, Node};
use crate::transpiler::ParseError;
use crate::{Lexer, Token};

#[derive(Debug)]
pub enum Line<'a> {
    Expr(Expr<'a>),
    Return(Option<Expr<'a>>),
    VarDecl(VarDecl<'a>),
}

impl<'a> Node<'a> for Line<'a> {
    fn generate(&self, content: &mut String) {
        todo!()
    }

    fn valid(&self) -> bool {
        match self {
            Line::Expr(v) => v.valid(),
            Line::Return(None) => true,
            Line::Return(Some(v)) => v.valid(),
            Line::VarDecl(v) => v.valid(),
        }
    }
}

impl<'a> Line<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        match lexer.cur_next() {
            None => return Err(ParseError::PrematureEOF),
            Some(Token::Textual("return")) => match lexer.cur() {
                None => return Err(ParseError::PrematureEOF),
                Some(Token::Special(";")) => {
                    lexer.next_cur();
                    Ok(Line::Return(None))
                }
                _ => Ok(Line::Return(Some(Expr::parse(lexer)?))),
            },
            Some(Token::Textual("var")) => Ok(Line::VarDecl(VarDecl::parse(lexer)?)),
            Some(v) => Ok(Line::Expr(Expr::parse(lexer)?)),
        }
    }
}
