use crate::ast::evaluative::Block;
use crate::ast::evaluative::Expr;
use crate::ast::typical::Type;
use crate::ast::{Ident, Node};
use crate::transpiler::ParseError;
use crate::Lexer;

#[derive(Debug)]
pub enum Literal<'a> {
    Array {
        items: Vec<Expr<'a>>,
    },
    Floating {
        value: FloatingLiteral,
    },
    Integer {
        value: IntegerLiteral,
    },
    Lambda {
        args: Vec<LambdaArg<'a>>,
        body: Box<Block<'a>>,
    },
    Range {
        from: Box<Expr<'a>>,
        to: Box<Expr<'a>>,
        openness: RangeOpenness,
    },
    String {
        value: &'a str,
    },
    Tuple {
        items: Vec<Expr<'a>>,
    },
}

#[derive(Debug)]
pub enum FloatingLiteral {
    F32(f32),
    F64(f64),
}

#[derive(Debug)]
pub enum IntegerLiteral {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

#[derive(Debug)]
pub struct LambdaArg<'a> {
    ident: Ident<'a>,
    type_hint: Option<Type<'a>>,
}

#[derive(Debug)]
pub enum RangeOpenness {
    Closed,
    HalfOpenLower,
    HalfOpenUpper,
    Open,
}

impl<'a> Node<'a> for Literal<'a> {
    fn generate(&self, content: &mut String) {}

    fn valid(&self) -> bool {
        true
    }

    fn parse(lexer: &mut Lexer<'a>) -> Result<Self, ParseError<'a>>
    where
        Self: Sized,
    {
        Err(ParseError::PrematureEOF)
    }
}
