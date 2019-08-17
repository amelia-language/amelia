#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Keyword {
    Module,
    If,
    Then,
    Else,
    Public,
    Do,
    End,
    As,
    Struct,
    Implement,
    Inherits,
}