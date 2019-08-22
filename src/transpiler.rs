
use crate::ast::Node;
use crate::token::TokenKind;
use crate::keyword::Keyword;

pub fn transpile(ast: Node) -> String {
    let mut syntax = vec![];
    for node in ast.children {
        if let Some(node_data) = &node.data {
            syntax.push(
                match node.token.kind {
                    TokenKind::keyword(Keyword::Module) => "mod",
                    TokenKind::keyword(Keyword::Public) => "pub",
                    TokenKind::keyword(Keyword::Trait) => "trait",
                    TokenKind::keyword(Keyword::As) => ":",
                    TokenKind::keyword(Keyword::Do) => "{",
                    TokenKind::keyword(Keyword::End) => "}",
                    TokenKind::keyword(Keyword::Derive) => "derive",
                    TokenKind::OpenParen => "(",
                    TokenKind::CloseParen => ")",
                    TokenKind::Dot => ".",
                    TokenKind::NewLine => ";",
                    TokenKind::keyword(Keyword::Function) => "fn",
                    TokenKind::Whitespace => " ",
                    TokenKind::Identifier => &node_data,
                    _ => ""
                }.to_string()
            );
        }
        syntax.push(transpile(node));
    }
    syntax.join("")
}
