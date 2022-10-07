use std::fmt::Debug;

pub trait Node<'a>: Debug {
    fn generate(&self, content: &mut String);
    fn valid(&self) -> bool;
}
