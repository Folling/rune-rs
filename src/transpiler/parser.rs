pub use crate::ast::Ast;
use crate::ast::{Node, TopLevelExpr};
pub use crate::transpiler::errors::{ParseErr, ParseErrorInfo};
pub use crate::transpiler::Lexer;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Parser<'a> {
        Parser {
            lexer: Lexer::new(content),
        }
    }

    pub fn parse(&mut self) -> Result<Ast<'a>, ParseErrorInfo<'a>> {
        let mut ast = Ast::new();

        self.lexer.next_cur();

        while self.lexer.cur().is_some() {
            ast.root_nodes.push(match TopLevelExpr::parse(&mut self.lexer) {
                Ok(val) => val,
                Err(err) => {
                    return Err(ParseErrorInfo {
                        line: self.lexer.pos().1,
                        column: self.lexer.pos().0,
                        err,
                    });
                }
            })
        }

        Ok(ast)
    }
}
