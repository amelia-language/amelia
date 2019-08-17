pub enum Lexeme {
    String(String)
    Int32(i32),
    Int64(i64),
    UInt32(u32),
    UInt64(u64),
    Float64(f64),
    Float32(f32),
    Usize(usize),
    Isize(isize),
    Char(char),
    Byte(u8),
}