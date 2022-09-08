#[derive(Debug)]
pub enum NotNeither<T> {
    Left(T),
    Right(T),
    Both(T, T),
}
