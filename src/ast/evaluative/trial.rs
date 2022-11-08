use crate::ast::evaluative::Expr;
use crate::ast::Node;
use crate::transpiler::ParseErr;

#[derive(Debug)]
pub struct TrialExpr<'a> {
    pub expr: Box<Expr<'a>>,
}

impl<'a> Node<'a> for TrialExpr<'a> {
    fn generate(&self, content: &mut String) {
        todo!()
    }

    fn valid(&self) -> bool {
        todo!()
    }
}

impl<'a> TrialExpr<'a> {
    pub fn parse(expr: Expr<'a>) -> Result<Self, ParseErr<'a>> {
        todo!()
    }
}
