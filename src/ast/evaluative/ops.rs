use crate::ast::evaluative::Expr;
use std::cell::LazyCell;
use std::collections::HashMap;
use std::fmt::Binary;

// all operators are taken to be left-associative
// if you want right-associativity, use parentheses

pub struct UnaryOp {
    ident: &'static str,
    func_ident: &'static str,
    prec: usize,
}

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
pub struct UnaryOpExpr<'a> {
    expr: Expr<'a>,
    op: UnaryOp,
}

#[derive(Debug)]
pub struct BinaryOpExpr<'a> {
    left: Expr<'a>,
    right: Expr<'a>,
    op: BinaryOp,
}
