#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token<'a> {
    Special(&'a str),
    Textual(&'a str),
}
