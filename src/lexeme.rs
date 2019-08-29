#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Lexeme {
    String,
    Int32,
    Int64,
    UInt32,
    UInt64,
    Float64,
    Float32,
    Usize,
    Isize,
    Char,
    Byte,
}
