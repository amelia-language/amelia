use crate::token;
use crate::keyword;

pub const DO: &str = "do";
pub const END: &str = "end";
pub const OPEN_PAREN: &str = "(";
pub const CLOSE_PAREN: &str = ")";
pub const OPEN_BRACKET: &str = "[";
pub const CLOSE_BRACKET: &str = "]";
pub const OPEN_BRACE: &str = "{";
pub const CLOSE_BRACE: &str = "}";

pub fn match_block_begin(token_kind: &token::TokenKind, begin_mark: &str) -> bool {
    (token_kind == &token::TokenKind::Keyword(keyword::Keyword::Do) && begin_mark == DO) ||
    (token_kind == &token::TokenKind::OpenParen && begin_mark == OPEN_PAREN)
}

pub fn match_block_end(token_kind: &token::TokenKind, begin_mark: &str) -> bool {
    (token_kind == &token::TokenKind::Keyword(keyword::Keyword::End) && begin_mark == DO) ||
    (token_kind == &token::TokenKind::CloseParen && begin_mark == OPEN_PAREN)
}

