use crate::keyword;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    line: i32
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub eos: bool
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    Root,
    LineComment,
    BlockComment,
    Whitespace,
    Identifier,
    IdentifierEnd,
    NewLine,
    Literal(LiteralKind),
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
    Keyword(keyword::Keyword),
    TypeWithGeneric,
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
    pub fn new(kind: TokenKind, line: i32, eos: bool) -> Token {
        Token { kind, span: Span { line }, eos }
    }
}
