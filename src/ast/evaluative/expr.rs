use crate::ast::evaluative::{BinaryOpExpr, CharLit, IfElseChain, LambdaLiteral, NumericLit, StringLit, TupleLit, UnaryOpExpr};
use crate::ast::{Ident, Node};
use crate::transpiler::ParseError;
use crate::{Lexer, Token};

#[derive(Debug)]
pub enum Expr<'a> {
    CharLiteral(CharLit<'a>),
    StringLiteral(StringLit<'a>),
    NumericLiteral(NumericLit),
    TupleLiteral(TupleLit<'a>),
    IfElseChain(IfElseChain<'a>),
    UnaryOp(UnaryOpExpr<'a>),
    BinaryOp(BinaryOpExpr<'a>),
    TrialOp(TrialOp<'a>),
    FuncCall(FuncCall<'a>),
}

impl<'a> Node<'a> for Expr<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        match self {
            Expr::CharLiteral(lit) => lit.valid(),
            Expr::StringLiteral(lit) => lit.valid(),
            Expr::NumericLiteral(lit) => lit.valid(),
            Expr::TupleLiteral(lit) => lit.valid(),
            Expr::IfElseChain(chain) => chain.valid(),
            Expr::UnaryOp(op) => op.valid(),
            Expr::BinaryOp(op) => op.valid(),
            Expr::TrialOp(op) => op.valid(),
            Expr::FuncCall(call) => call.valid(),
        }
    }
}

impl<'a> Expr<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        match lexer.cur_next() {
            None => return Err(ParseError::PrematureEOF),
            Some(Token::Special("'")) => Ok(Expr::CharLiteral(CharLit::parse(lexer)?)),
            Some(Token::Special("\"")) => Ok(Expr::StringLiteral(StringLit::parse(lexer)?)),
            Some(Token::Textual(v)) if matches!(v.chars().next(), Some('0'..'9')) => Ok(Expr::NumericLiteral(NumericLit::parse(lexer)?)),
            Some(Token::Special("[")) => Ok(Expr::TupleLiteral(TupleLit::parse(lexer)?)),
            Some(v) => todo!(),
        }
    }
}
