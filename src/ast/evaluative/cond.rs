use crate::ast::evaluative::{Block, Expr};
use crate::ast::Node;
use crate::transpiler::ParseError;
use crate::Lexer;

#[derive(Debug)]
pub struct CondBlock<'a> {
    cond: Expr<'a>,
    block: Block<'a>,
}

#[derive(Debug)]
pub struct IfElseChain<'a> {
    r#if: CondBlock<'a>,
    r#else_ifs: Vec<CondBlock<'a>>,
    r#else: Option<CondBlock<'a>>,
}

impl<'a> Node<'a> for IfElseChain<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        self.r#if.valid() && self.else_ifs.iter().all(Node::valid) && matches!(self.r#else.as_ref(), Some(x) if x.valid())
    }

    fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl<'a> Node<'a> for CondBlock<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        self.cond.valid() && self.block.valid()
    }

    fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        todo!()
    }
}
