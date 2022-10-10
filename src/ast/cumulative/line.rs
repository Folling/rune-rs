use crate::ast::evaluative::{Expr, VarDecl};
use crate::ast::Node;
use crate::transpiler::{util, ParseErr};
use crate::{Lexer, Token};

#[derive(Debug)]
pub enum Line<'a> {
    Expr(Expr<'a>),
    Return(Option<Expr<'a>>),
    VarDecl(VarDecl<'a>),
}

impl<'a> Node<'a> for Line<'a> {
    fn generate(&self, content: &mut String) {
        todo!()
    }

    fn valid(&self) -> bool {
        match self {
            Line::Expr(val) => val.valid(),
            Line::Return(None) => true,
            Line::Return(Some(val)) => val.valid(),
            Line::VarDecl(val) => val.valid(),
        }
    }
}

impl<'a> Line<'a> {
    pub fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseErr<'a>>
    where
        Self: Sized,
    {
        let line = match lexer.cur_next() {
            None => return Err(ParseErr::PrematureEOF),
            Some((Token::Textual("return"), _)) => match lexer.cur() {
                None => return Err(ParseErr::PrematureEOF),
                Some((Token::Special(";"), _)) => Line::Return(None),
                _ => Line::Return(Some(Expr::parse(lexer)?)),
            },
            Some((Token::Textual("var"), _)) => Line::VarDecl(VarDecl::parse(lexer)?),
            Some(_) => Line::Expr(Expr::parse(lexer)?),
        };

        util::exp_cur_next_sp_tok(lexer, ";")?;

        Ok(line)
    }
}
