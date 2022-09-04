use crate::ast::evaluative::Expr;
use crate::ast::typical::Type;
use crate::ast::{Ident, Node};
use crate::transpiler::{util, ParseError};
use crate::{Lexer, Token};

#[derive(Debug)]
pub struct VarDecl<'a> {
    ident: Ident<'a>,
    r#type: Option<Type<'a>>,
    assignment: Option<Expr<'a>>,
}

impl<'a> Node<'a> for VarDecl<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        self.ident.valid()
            && (self.r#type.is_none() || matches!(&self.r#type, Some(v) if v.valid()))
            && (self.assignment.is_none() || matches!(&self.assignment, Some(v) if v.valid()))
    }

    fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        let ident = Ident::parse(lexer)?;

        let r#type = match lexer.cur() {
            None => return Err(ParseError::PrematureEOF),
            Some(Token::Special(":")) => {
                lexer.next_cur();
                Some(Type::parse(lexer)?)
            }
            Some(_) => None,
        };

        let assignment = match lexer.cur() {
            None => return Err(ParseError::PrematureEOF),
            Some(Token::Special(";")) => None,
            Some(Token::Special("=")) => {
                lexer.next_cur();
                Some(Expr::parse(lexer)?)
            }
            Some(v) => {
                return Err(ParseError::InvalidToken {
                    got: v,
                    expected: vec![Token::Special(";"), Token::Special("=")],
                })
            }
        };

        Ok(VarDecl { ident, r#type, assignment })
    }
}
