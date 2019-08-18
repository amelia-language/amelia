use crate::keyword;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    line: i32,
    col: i32
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    LineComment,
    BlockComment { terminated: bool },
    Whitespace,
    Ident,
    RawIdent,
    Literal,
    Lifetime,
    Semi,
    Comma,
    DotDotDot,
    DotDotEq,
    DotDot,
    Dot,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    At,
    Pound,
    Tilde,
    Question,
    ColonColon,
    Colon,
    Dollar,
    EqEq,
    Eq,
    FatArrow,
    Ne,
    Not,
    Le,
    LArrow,
    Lt,
    ShlEq,
    Shl,
    Ge,
    Gt,
    ShrEq,
    Shr,
    RArrow,
    Minus,
    MinusEq,
    And,
    AndAnd,
    AndEq,
    Or,
    OrOr,
    OrEq,
    PlusEq,
    Plus,
    StarEq,
    Star,
    SlashEq,
    Slash,
    CaretEq,
    Caret,
    PercentEq,
    Percent,
    keyword(keyword::Keyword),
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
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
    String,
}

impl Token {
    pub fn new(kind: TokenKind, line: i32, col: i32) -> Token {
        Token { kind, span: Span { line, col } }
    }
}