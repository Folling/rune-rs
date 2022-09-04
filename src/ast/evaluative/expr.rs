use crate::ast::{Ident, Node};
use crate::transpiler::ParseError;
use crate::Lexer;

#[derive(Debug)]
pub enum Expr<'a> {
    FuncCall {
        ident: Ident<'a>,
        args: Vec<Expr<'a>>,
    },
    MethodCall {
        subj: Ident<'a>,
        ident: Ident<'a>,
        args: Vec<Expr<'a>>,
    },
}

impl<'a> Node<'a> for Expr<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        match self {
            Expr::FuncCall { ident, args } => ident.valid() && args.iter().all(Node::valid),
            Expr::MethodCall { subj, ident, args } => subj.valid() && ident.valid() && args.iter().all(Node::valid),
        }
    }

    fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        Ok(Expr::FuncCall {
            ident: Ident {
                raw: false,
                name: "println",
            },
            args: vec![],
        })
    }
}
