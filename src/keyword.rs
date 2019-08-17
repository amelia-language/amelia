#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Keyword {
    If,
    Then,
    Else,
    For,
    Do,
    End,
    Public,
    Struct,
    Implements,
    Inherits,
    As,
    Equal,
    Function,
    Mutable,
    Borrow,
    Own,
    In,
    Optional,
    Let,
    Module,
    Derive
}