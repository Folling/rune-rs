pub use crate::ast::Node;
use crate::ast::TopLevelExpr;

#[derive(Debug)]
pub struct Ast<'a> {
    pub root_nodes: Vec<TopLevelExpr<'a>>,
}

impl<'a> Ast<'a> {
    pub fn new() -> Ast<'a> {
        Ast { root_nodes: vec![] }
    }
}
