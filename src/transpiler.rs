use std::str;

use crate::ast::Node;
use crate::token::TokenKind;
use crate::keyword::Keyword;

pub fn transpile(ast: Node) -> String {
    let mut syntax = vec![];
    for node in ast.children {
        if let Some(node_data) = &node.data {
            syntax.push(
                match node.token.kind {
                    TokenKind::keyword(Keyword::Module) => "mod".to_string(),
                    TokenKind::keyword(Keyword::Public) => "pub".to_string(),
                    TokenKind::keyword(Keyword::Trait) => "trait".to_string(),
                    TokenKind::keyword(Keyword::As) => ":".to_string(),
                    TokenKind::keyword(Keyword::Do) => "{".to_string(),
                    TokenKind::keyword(Keyword::End) => "}".to_string(),
                    TokenKind::keyword(Keyword::Derive) => "derive".to_string(),
                    TokenKind::OpenParen => "(".to_string(),
                    TokenKind::CloseParen => ")".to_string(),
                    TokenKind::Dot => ".".to_string(),
                    TokenKind::keyword(Keyword::Function) => "fn".to_string(),
                    TokenKind::Whitespace => " ".to_string(),
                    TokenKind::Identifier => {
                        let identifier = str::replace(node_data, "\n", ";");
                        identifier
                    },
                    _ => "".to_string()
                }
            );
        }
        syntax.push(transpile(node));
    }
    syntax.join("")
}
