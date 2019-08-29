use std::str;

use crate::ast::Node;
use crate::token::{ TokenKind, LiteralKind };
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
                    TokenKind::TypeWithGeneric => str::replace(node_data, "\n", ";\n"),
                    TokenKind::Keyword(Keyword::As) => ":".to_string(),
                    TokenKind::Keyword(Keyword::If) => "if".to_string(),
                    TokenKind::Keyword(Keyword::Else) => "} else".to_string(),
                    TokenKind::Keyword(Keyword::Do) => "{".to_string(),
                    TokenKind::Keyword(Keyword::End) => "}".to_string(),
                    TokenKind::Keyword(Keyword::Derive) => format!("#[{}]", node_data),
                    TokenKind::Keyword(Keyword::Implements) => "impl".to_string(),
                    TokenKind::Keyword(Keyword::For) => "for".to_string(),
                    TokenKind::Macro => node_data.to_string(),
                    TokenKind::OpenParen => "(".to_string(),
                    TokenKind::CloseParen => str::replace(node_data, "\n", ";\n"),
                    TokenKind::Dot => ".".to_string(),
                    TokenKind::Keyword(Keyword::Function) => "fn".to_string(),
                    TokenKind::Keyword(Keyword::PublicFunction) => "pub fn".to_string(),
                    TokenKind::Whitespace => " ".to_string(),
                    TokenKind::NewLine => "\n".to_string(),
                    TokenKind::Comma => ",".to_string(),
                    TokenKind::Identifier => str::replace(node_data, "\n", ";\n"),
                    TokenKind::Literal(LiteralKind::Boolean) => {
                        let data_type = str::replace(node_data, "\n", ";\n");
                        data_type.replace("Boolean", "bool")
                    },
                    TokenKind::Literal(LiteralKind::String) => str::replace(node_data, "\n", ";\n"),
                    TokenKind::Literal(LiteralKind::Int32) =>{
                        let data_type = str::replace(node_data, "\n", ";\n");
                        data_type.replace("Int32", "i32")
                    },
                    TokenKind::Literal(LiteralKind::Int64) =>{
                        let data_type = str::replace(node_data, "\n", ";\n");
                        data_type.replace("Int64", "i64")
                    },
                    TokenKind::Literal(LiteralKind::UInt32) => {
                        let data_type = str::replace(node_data, "\n", ";\n");
                        data_type.replace("UInt32", "u32")
                    },
                    TokenKind::Literal(LiteralKind::UInt64) => {
                        let data_type = str::replace(node_data, "\n", ";\n");
                        data_type.replace("UInt64", "u64")
                    },
                    TokenKind::Literal(LiteralKind::Float64) => {
                        let data_type = str::replace(node_data, "\n", ";\n");
                        data_type.replace("Float64", "f64")
                    },
                    TokenKind::Literal(LiteralKind::Float32) => {
                        let data_type = str::replace(node_data, "\n", ";\n");
                        data_type.replace("Float32", "f32")
                    },
                    TokenKind::Literal(LiteralKind::Usize) => {
                        let data_type = str::replace(node_data, "\n", ";\n");
                        data_type.replace("Float32", "f32")
                    },
                    TokenKind::Literal(LiteralKind::Isize) => {
                        let data_type = str::replace(node_data, "\n", ";\n");
                        data_type.replace("Isize", "isize")
                    },
                    TokenKind::Literal(LiteralKind::Byte) => {
                        let data_type = str::replace(node_data, "\n", ";\n");
                        data_type.replace("Byte", "u8")
                    },
                    _ => "".to_string()
                }
            );
        }
        syntax.push(transpile(node));
    }
    syntax.join("")
}
