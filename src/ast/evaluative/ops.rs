use crate::ast::evaluative::{Expr, NumLit};
use crate::ast::Node;
use crate::transpiler::ParseErr;
use std::cell::LazyCell;
use std::collections::HashMap;

// all operators are taken to be left-associative
// if you want right-associativity, use parentheses

#[derive(Debug)]
pub struct UnaryOp {
    ident: &'static str,
    func_ident: &'static str,
    prec: usize,
}

#[derive(Debug)]
pub struct BinaryOp {
    ident: &'static str,
    func_ident: &'static str,
    prec: usize,
}

pub const UNARY_OPS: LazyCell<HashMap<&'static str, UnaryOp>> = LazyCell::new(|| {
    let mut ret = HashMap::with_capacity(64);

    let insert = |ident, func_ident, prec| ret.insert(ident, UnaryOp { ident, func_ident, prec });

    insert("!", "inv", 0);
    insert("+", "pos", 0);
    insert("-", "neg", 0);

    ret
});

pub const BINARY_OPS: LazyCell<HashMap<&'static str, BinaryOp>> = LazyCell::new(|| {
    let mut ret = HashMap::with_capacity(64);

    let binsert = |ident, func_ident, prec| ret.insert(ident, BinaryOp { ident, func_ident, prec });

    binsert("<<", "shl", 1);
    binsert(">>", "shr", 1);

    binsert("**", "pow", 2);
    binsert("//", "root", 2);

    binsert("*", "mul", 3);
    binsert("/", "div", 3);
    binsert("%", "mod", 3);

    binsert("+", "add", 4);
    binsert("-", "sub", 4);

    binsert("&", "band", 5);
    binsert("^", "bxor", 6);
    binsert("|", "bor", 7);

    binsert("<", "lt", 8);
    binsert("<=", "le", 8);
    binsert(">", "gt", 8);
    binsert(">=", "ge", 8);

    binsert("==", "eq", 9);
    binsert("!=", "eq", 9);

    binsert("&&", "and", 10);
    binsert("||", "or", 11);
    binsert("^^", "xor", 12);

    ret
});

#[derive(Debug)]
pub struct UnOpExpr<'a> {
    expr: Box<Expr<'a>>,
    op: UnaryOp,
}

impl<'a> Node<'a> for UnOpExpr<'a> {
    fn generate(&self, content: &mut String) {
        todo!()
    }

    fn valid(&self) -> bool {
        todo!()
    }
}

impl<'a> UnOpExpr<'a> {
    pub fn parse(expr: Expr<'a>) -> Result<Self, ParseErr<'a>> {
        Ok(UnOpExpr {
            expr: Box::new(expr),
            op: UnaryOp {
                ident: "+",
                func_ident: "pos",
                prec: 2,
            },
        })
    }
}

#[derive(Debug)]
pub struct BinOpExpr<'a> {
    left: Box<Expr<'a>>,
    right: Box<Expr<'a>>,
    op: BinaryOp,
}

impl<'a> Node<'a> for BinOpExpr<'a> {
    fn generate(&self, content: &mut String) {
        todo!()
    }

    fn valid(&self) -> bool {
        todo!()
    }
}

impl<'a> BinOpExpr<'a> {
    pub fn parse(expr: Expr<'a>) -> Result<Self, ParseErr<'a>> {
        Ok(BinOpExpr {
            left: Box::new(expr),
            right: Box::new(Expr::NumLit(NumLit { value: "5", r#type: "u32" })),
            op: BinaryOp {
                ident: "+",
                func_ident: "add",
                prec: 4,
            },
        })
    }
}
