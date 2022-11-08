use crate::ast::evaluative::{Block, Expr};
use crate::ast::Node;
use crate::transpiler::{ExpectedToken, ParseErr};
use crate::{Lexer, Token};

#[derive(Debug)]
pub struct CondBlock<'a> {
    cond: Box<Expr<'a>>,
    block: Block<'a>,
}

#[derive(Debug)]
pub struct IfElseChain<'a> {
    r#if: CondBlock<'a>,
    r#else_ifs: Vec<CondBlock<'a>>,
    r#else: Option<Block<'a>>,
}

impl<'a> Node<'a> for IfElseChain<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        self.r#if.valid() && self.else_ifs.iter().all(Node::valid) && matches!(self.r#else.as_ref(), Some(x) if x.valid())
    }
}

impl<'a> IfElseChain<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseErr<'a>>
    where
        Self: Sized,
    {
        let r#if = CondBlock {
            cond: Box::new(Expr::parse(lexer)?),
            block: Block::parse(lexer, false)?,
        };

        let mut else_ifs = Vec::with_capacity(4);

        let mut r#else = None;

        loop {
            match lexer.cur() {
                None => return Err(ParseErr::PrematureEOF),
                Some((Token::Textual("else"), _)) => {
                    lexer.skip();

                    match lexer.cur() {
                        None => return Err(ParseErr::PrematureEOF),
                        Some((Token::Special("{"), _)) => {
                            r#else = Some(Block::parse(lexer, false)?);
                        }
                        Some((Token::Textual("if"), _)) => {
                            lexer.skip();
                            else_ifs.push(CondBlock {
                                cond: Box::new(Expr::parse(lexer)?),
                                block: Block::parse(lexer, false)?,
                            });
                        }
                        Some((v, _)) => {
                            return Err(ParseErr::InvalidToken {
                                got: v,
                                expected: vec![ExpectedToken::Textual { regex: "if" }, ExpectedToken::Special { regex: "{" }],
                            })
                        }
                    }
                }
                Some((Token::Special(";"), _)) => return Ok(Self { r#if, else_ifs, r#else }),
                Some((v, _)) => {
                    return Err(ParseErr::InvalidToken {
                        got: v,
                        expected: vec![
                            ExpectedToken::Textual { regex: "else" },
                            ExpectedToken::Textual { regex: "else if" },
                            ExpectedToken::Special { regex: ";" },
                        ],
                    })
                }
            }
        }
    }
}

impl<'a> Node<'a> for CondBlock<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        self.cond.valid() && self.block.valid()
    }
}

impl<'a> CondBlock<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseErr<'a>>
    where
        Self: Sized,
    {
        todo!()
    }
}
