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
                    TokenKind::Keyword(Keyword::PublicTrait) => "pub trait".to_string(),
                    TokenKind::Keyword(Keyword::Struct) => {
                        "#[derive(Debug, Clone, Default, PartialEq)]\nstruct".to_string()
                    },
                    TokenKind::Keyword(Keyword::PublicStruct) => {
                        "#[derive(Debug, Clone, Default, PartialEq)]\npub struct".to_string()
                    },
                    TokenKind::Keyword(Keyword::Enum) => {
                        "#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]\nenum".to_string()
                    },
                    TokenKind::Keyword(Keyword::PublicEnum) => {
                        "#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]\npub enum".to_string()
                    },
                    TokenKind::TypeWithGeneric => {
                        let identifier = str::replace(node_data, "\n", ";\n");
                        identifier
                    },
                    TokenKind::Keyword(Keyword::As) => ":".to_string(),
                    TokenKind::Keyword(Keyword::Do) => "{".to_string(),
                    TokenKind::Keyword(Keyword::End) => "}".to_string(),
                    TokenKind::Keyword(Keyword::Derive) => format!("#[{}]", node_data),
                    TokenKind::Keyword(Keyword::Implements) => "impl".to_string(),
                    TokenKind::Keyword(Keyword::For) => "for".to_string(),
                    TokenKind::OpenParen => "(".to_string(),
                    TokenKind::CloseParen => ")".to_string(),
                    TokenKind::Dot => ".".to_string(),
                    TokenKind::Keyword(Keyword::Function) => "fn".to_string(),
                    TokenKind::Keyword(Keyword::PublicFunction) => "pub fn".to_string(),
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
