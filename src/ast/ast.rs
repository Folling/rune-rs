pub use crate::ast::Node;
use crate::ast::TopLevelExpression;

#[derive(Debug)]
pub struct Ast<'a> {
    pub root_nodes: Vec<TopLevelExpression<'a>>,
}

impl<'a> Ast<'a> {
    pub fn new() -> Ast<'a> {
        Ast { root_nodes: vec![] }
    }
}
