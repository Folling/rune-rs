use crate::ast::evaluative::{
    BinOpExpr, CharLit, FuncCall, IfElseChain, NumLit, RangeLit, RangeOpenness, StrLit, TrialExpr, TupleLit, UnOpExpr,
};
use crate::ast::Node;
use crate::transpiler::{ParseErr, TokenLoc};
use crate::{Lexer, Token};

#[derive(Debug)]
pub enum Expr<'a> {
    CharLit(CharLit<'a>),
    StrLit(StrLit<'a>),
    NumLit(NumLit<'a>),
    RangeLit(RangeLit<'a>),
    TupleLit(TupleLit<'a>),
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
            Expr::CharLit(lit) => lit.valid(),
            Expr::StrLit(lit) => lit.valid(),
            Expr::NumLit(lit) => lit.valid(),
            Expr::RangeLit(lit) => lit.valid(),
            Expr::TupleLit(lit) => lit.valid(),
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
        let mut expr = match lexer.cur_next() {
            None => return Err(ParseErr::PrematureEOF),
            Some((Token::Special("'"), _)) => Expr::CharLit(CharLit::parse(lexer)?),
            Some((Token::Special("\""), _)) => Expr::StrLit(StrLit::parse(lexer)?),
            Some((Token::Textual(val), loc)) if matches!(val.chars().next(), Some('0'..='9')) => {
                Expr::NumLit(NumLit::parse(lexer, val, loc)?)
            }
            Some((Token::Textual("if"), _)) => Expr::IfElseChain(IfElseChain::parse(lexer)?),
            Some((Token::Special("["), _)) => Expr::TupleLit(TupleLit::parse(lexer)?),
            Some((Token::Textual(v), _)) => {}
        };

        loop {
            expr = match lexer.cur() {
                Some((Token::Special(".."), _)) => {
                    lexer.skip();

                    Expr::RangeLit(RangeLit {
                        from: Box::new(expr),
                        to: Box::new(Expr::parse(lexer)?),
                        openness: RangeOpenness::Open,
                    })
                }
                Some((Token::Special(">.."), _)) => {
                    lexer.skip();

                    Expr::RangeLit(RangeLit {
                        from: Box::new(expr),
                        to: Box::new(Expr::parse(lexer)?),
                        openness: RangeOpenness::HalfOpenUpper,
                    })
                }
                Some((Token::Special("..<"), _)) => {
                    lexer.skip();

                    Expr::RangeLit(RangeLit {
                        from: Box::new(expr),
                        to: Box::new(Expr::parse(lexer)?),
                        openness: RangeOpenness::HalfOpenLower,
                    })
                }
                Some((Token::Special(">..<"), _)) => {
                    lexer.skip();

                    Expr::RangeLit(RangeLit {
                        from: Box::new(expr),
                        to: Box::new(Expr::parse(lexer)?),
                        openness: RangeOpenness::Closed,
                    })
                }
                Some((Token::Special("?"), _)) => {
                    lexer.skip();
                    Expr::TrialExpr(TrialExpr { expr: Box::new(expr) })
                }
                _ => break,
            }
        }

        Ok(expr)
    }
}
