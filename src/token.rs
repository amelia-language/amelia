use crate::keyword;
use crate::lexeme;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    line: i32
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub eos: bool
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum TokenKind {
    Assign,
    Root,
    LineComment,
    BlockComment,
    Whitespace,
    Identifier,
    IdentifierEnd,
    NewLine,
    Literal(LiteralKind),
    Lexeme(lexeme::Lexeme),
    Operator(Operator),
    Collection(Collection),
    Lifetime,
    Semi,
    Comma,
    DotDotDot,
    DotDotEq,
    DotDot,
    DoubleDot,
    NamespaceSeparator,
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
    Equal,
    NotEqual,
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
    Macro,
    MacroBody,
    FunctionCall,
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
    Boolean
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Add,
    Minus,
    Divide,
    Multiply,
    Mod
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Collection {
    Array,
    HashMap,
    Tuple,
    Vector
}

impl Token {
    pub fn new(kind: TokenKind, line: i32, eos: bool) -> Token {
        Token { kind, span: Span { line }, eos }
    }
}
