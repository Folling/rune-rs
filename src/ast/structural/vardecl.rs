use crate::ast::evaluative::Expr;
use crate::ast::typical::Type;
use crate::ast::{Ident, Node};
use crate::transpiler::{util, ExpectedToken, ParseErr};
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
            && (self.r#type.is_none() || matches!(&self.r#type, Some(val) if val.valid()))
            && (self.assignment.is_none() || matches!(&self.assignment, Some(val) if val.valid()))
    }
}

impl<'a> VarDecl<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseErr<'a>>
    where
        Self: Sized,
    {
        let ident = Ident::parse(lexer)?;

        let r#type = match lexer.cur() {
            None => return Err(ParseErr::PrematureEOF),
            Some((Token::Special(":"), _)) => {
                lexer.next_cur();
                Some(Type::parse(lexer)?)
            }
            Some(_) => None,
        };

        let assignment = match lexer.cur() {
            None => return Err(ParseErr::PrematureEOF),
            Some((Token::Special(";"), _)) => None,
            Some((Token::Special("="), _)) => {
                lexer.next_cur();
                Some(Expr::parse(lexer)?)
            }
            Some(val) => {
                return Err(ParseErr::InvalidToken {
                    got: val,
                    expected: vec![ExpectedToken::Special { regex: ";" }, ExpectedToken::Special { regex: "=" }],
                })
            }
        };

        Ok(VarDecl { ident, r#type, assignment })
    }
}
