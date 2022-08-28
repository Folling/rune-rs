#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token<'a> {
    EOF,
    Special(&'a str),
    Textual(&'a str),
}
