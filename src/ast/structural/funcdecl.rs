use crate::ast::evaluative::Block;
use crate::ast::structural::FuncArg;
use crate::ast::typical::Type;
use crate::ast::{Ident, Node};
use crate::transpiler::{util, ExpectedToken, ParseErr};
use crate::{Lexer, Token};
use std::error::Error;

#[derive(Debug)]
pub struct FuncProto<'a> {
    pub(crate) ident: Ident<'a>,
    pub(crate) args: Vec<FuncArg<'a>>,
    pub(crate) ret: Type<'a>,
}

#[derive(Debug)]
pub struct FuncDecl<'a> {
    pub(crate) proto: FuncProto<'a>,
    pub(crate) body: Block<'a>,
}

impl<'a> Node<'a> for FuncDecl<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        self.proto.valid() && self.body.valid()
    }
}

impl<'a> FuncDecl<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseErr<'a>>
    where
        Self: Sized,
    {
        Ok(FuncDecl {
            proto: FuncProto::parse(lexer)?,
            body: Block::parse(lexer)?,
        })
    }
}

impl<'a> Node<'a> for FuncProto<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        self.ident.valid() && self.args.iter().all(Node::valid) && self.ret.valid()
    }
}

impl<'a> FuncProto<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseErr<'a>>
    where
        Self: Sized,
    {
        let ident = Ident::parse(lexer)?;

        util::exp_cur_next_sp_tok(lexer, "(")?;

        let mut args = vec![];

        loop {
            match lexer.cur() {
                None => return Err(ParseErr::PrematureEOF),
                Some((Token::Special(","), _)) => {
                    lexer.next_cur();
                    continue;
                }
                Some((Token::Special(")"), _)) => {
                    lexer.next_cur();
                    break;
                }
                Some((Token::Textual(_), _)) => args.push(FuncArg::parse(lexer)?),
                Some((val, _)) => {
                    return Err(ParseErr::InvalidToken {
                        got: val,
                        expected: vec![
                            ExpectedToken::Special { regex: "," },
                            ExpectedToken::Special { regex: ")" },
                            ExpectedToken::Textual { regex: "any text" },
                        ],
                    })
                }
            }
        }

        util::exp_cur_next_sp_tok(lexer, "->")?;

        let ret = Type::parse(lexer)?;

        Ok(FuncProto { ident, args, ret })
    }
}
