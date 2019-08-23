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
                    TokenKind::Keyword(Keyword::Module) => "mod".to_string(),
                    TokenKind::Keyword(Keyword::Public) => "pub".to_string(),
                    TokenKind::Keyword(Keyword::Trait) => "trait".to_string(),
                    TokenKind::Keyword(Keyword::As) => ":".to_string(),
                    TokenKind::Keyword(Keyword::Do) => "{".to_string(),
                    TokenKind::Keyword(Keyword::End) => "}".to_string(),
                    TokenKind::Keyword(Keyword::Derive) => format!("#[{}]", node_data),
                    TokenKind::OpenParen => "(".to_string(),
                    TokenKind::CloseParen => ")".to_string(),
                    TokenKind::Dot => ".".to_string(),
                    TokenKind::Keyword(Keyword::Function) => "fn".to_string(),
                    TokenKind::Whitespace => " ".to_string(),
                    TokenKind::NewLine => "\n".to_string(),
                    TokenKind::Comma => ",".to_string(),
                    TokenKind::Identifier => {
                        let identifier = str::replace(node_data, "\n", ";\n");
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
