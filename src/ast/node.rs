trait Node {
    fn generate(&self, str: &mut String);
    fn valid(&self) -> bool;
}
