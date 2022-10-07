use crate::ast::evaluative::{BinOpExpr, CharLit, FuncCall, IfElseChain, NumLit, StringLit, TrialExpr, TupleLit, UnOpExpr};
use crate::ast::Node;
use crate::transpiler::ParseErr;
use crate::{Lexer, Token};

#[derive(Debug)]
pub enum Expr<'a> {
    CharLiteral(CharLit<'a>),
    StringLiteral(StringLit<'a>),
    NumLit(NumLit<'a>),
    TupleLiteral(TupleLit<'a>),
    IfElseChain(IfElseChain<'a>),
    UnaryOp(UnOpExpr<'a>),
    BinaryOp(BinOpExpr<'a>),
    TrialExpr(TrialExpr<'a>),
    FuncCall(FuncCall<'a>),
}

impl<'a> Node<'a> for Expr<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        match self {
            Expr::CharLiteral(lit) => lit.valid(),
            Expr::StringLiteral(lit) => lit.valid(),
            Expr::NumLit(lit) => lit.valid(),
            Expr::TupleLiteral(lit) => lit.valid(),
            Expr::IfElseChain(chain) => chain.valid(),
            Expr::UnaryOp(op) => op.valid(),
            Expr::BinaryOp(op) => op.valid(),
            Expr::TrialExpr(expr) => expr.valid(),
            Expr::FuncCall(call) => call.valid(),
        }
    }
}

impl<'a> Expr<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseErr<'a>>
    where
        Self: Sized,
    {
        match lexer.cur_next() {
            None => return Err(ParseErr::PrematureEOF),
            Some((Token::Special("'"), _)) => Ok(Expr::CharLiteral(CharLit::parse(lexer)?)),
            Some((Token::Special("\""), _)) => Ok(Expr::StringLiteral(StringLit::parse(lexer)?)),
            Some((Token::Textual(val), loc)) if matches!(val.chars().next(), Some('0'..='9')) => {
                Ok(Expr::NumLit(NumLit::parse(lexer, val, loc)?))
            }
            Some((Token::Special("["), _)) => Ok(Expr::TupleLiteral(TupleLit::parse(lexer)?)),
            Some(_) => todo!(),
        }
    }
}
