use crate::ast::evaluative::Expr;
use crate::ast::Node;
use crate::transpiler::ParseErr;

#[derive(Debug)]
pub struct FuncCall<'a> {
    func_expr: Box<Expr<'a>>,
    args: Vec<Expr<'a>>,
}

impl<'a> Node<'a> for FuncCall<'a> {
    fn generate(&self, content: &mut String) {
        todo!()
    }

    fn valid(&self) -> bool {
        todo!()
    }
}

impl<'a> FuncCall<'a> {
    pub fn parse(expr: Expr<'a>) -> Result<Self, ParseErr<'a>> {
        Ok(FuncCall {
            func_expr: Box::new(expr),
            args: vec![],
        })
    }
}
