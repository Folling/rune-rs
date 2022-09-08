use crate::ast::evaluative::literal::Literal;
use crate::ast::{Ident, Node};
use crate::transpiler::ParseError;
use crate::{Lexer, Token};

#[derive(Debug)]
pub enum Expr<'a> {
    FuncCall {
        ident: Ident<'a>,
        args: Vec<Expr<'a>>,
    },
    MethodCall {
        subj: Box<Expr<'a>>,
        ident: Ident<'a>,
        args: Vec<Expr<'a>>,
    },
    Literal {
        value: Literal<'a>,
    },
}

impl<'a> Node<'a> for Expr<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        match self {
            Expr::FuncCall { ident, args } => ident.valid() && args.iter().all(Node::valid),
            Expr::MethodCall { subj, ident, args } => subj.valid() && ident.valid() && args.iter().all(Node::valid),
            Expr::Literal { value } => value.valid(),
        }
    }

    fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        match lexer.cur() {
            None => return Err(ParseError::PrematureEOF),
            Some(Token::Special("'")) => Ok(Expr::Literal {
                value: Literal::parse(lexer)?,
            }),
            Some(Token::Special("\"")) => Ok(Expr::Literal {
                value: Literal::parse(lexer)?,
            }),
            Some(Token::Special("[")) => Ok(Expr::Literal {
                value: Literal::parse(lexer)?,
            }),
            Some(Token::Special("|")) => Ok(Expr::Literal {
                value: Literal::parse(lexer)?,
            }),
            Some(Token::Textual(v)) if matches!(v.chars().next(), Some(c) if c.is_ascii_digit()) => Ok(Expr::Literal {
                value: Literal::parse(lexer)?,
            }),
            Some(v) => todo!(),
        }
    }
}
