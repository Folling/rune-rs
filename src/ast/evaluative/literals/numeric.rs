use crate::ast::Node;
use crate::transpiler::ParseError;
use crate::Lexer;

#[derive(Debug)]
pub enum IntV {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

#[derive(Debug)]
pub enum FloatV {
    F32(f32),
    F64(f64),
}

#[derive(Debug)]
pub enum NumericLit {
    Integral(IntV),
    Floating(FloatV),
}

impl<'a> Node<'a> for NumericLit {
    fn generate(&self, content: &mut String) {
        todo!()
    }

    fn valid(&self) -> bool {
        todo!()
    }
}
