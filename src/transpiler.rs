use std::str;

use crate::ast::Node;
use crate::token::{ TokenKind, LiteralKind, Operator, Collection };
use crate::keyword::Keyword;
use crate::lexeme::Lexeme;

pub fn transpile(ast: Node) -> String {
    let mut syntax = vec![];
    let mut new_line = true;
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
                    TokenKind::Keyword(Keyword::As) => ":".to_string(),
                    TokenKind::Keyword(Keyword::If) => "if".to_string(),
                    TokenKind::Keyword(Keyword::Else) => "} else".to_string(),
                    TokenKind::Keyword(Keyword::Do) => "{".to_string(),
                    TokenKind::Keyword(Keyword::End) => "}".to_string(),
                    TokenKind::Keyword(Keyword::Derive) => format!("#[{}]", node_data),
                    TokenKind::Keyword(Keyword::Implements) => "impl".to_string(),
                    TokenKind::Keyword(Keyword::For) => "for".to_string(),
                    TokenKind::Keyword(Keyword::Function) => "fn".to_string(),
                    TokenKind::Keyword(Keyword::PublicFunction) => "pub fn".to_string(),
                    TokenKind::Keyword(Keyword::Let) => "let".to_string(),
                    TokenKind::Keyword(Keyword::Borrow) => "&".to_string(),
                    TokenKind::Keyword(Keyword::Own) => "*".to_string(),
                    TokenKind::TypeWithGeneric => str::replace(node_data, "\n", ";\n"),
                    TokenKind::Macro => node_data.to_string(),
                    TokenKind::Equal => "==".to_string(),
                    TokenKind::Not => "!".to_string(),
                    TokenKind::NotEqual => "!=".to_string(),
                    TokenKind::OpenParen => "(".to_string(),
                    TokenKind::CloseParen => handle_new_line(&mut new_line, node_data),
                    TokenKind::Dot => ".".to_string(),
                    TokenKind::Whitespace => " ".to_string(),
                    TokenKind::NewLine => "\n".to_string(),
                    TokenKind::Comma => ",".to_string(),
                    TokenKind::Assign => "=".to_string(),
                    TokenKind::DoubleDot => ":".to_string(),
                    TokenKind::NamespaceSeparator => "::".to_string(),
                    TokenKind::Identifier => handle_new_line(&mut new_line, node_data),
                    TokenKind::Collection(Collection::Array) => handle_new_line(&mut new_line, node_data),
                    TokenKind::Collection(Collection::Tuple) => handle_new_line(&mut new_line, node_data),
                    TokenKind::Collection(Collection::HashMap) => {
                        let raw_items = node_data.split(",").collect::<Vec<_>>();
                        let mut items: Vec<String> = vec![];
                        for item in raw_items.iter() {
                            let replaced = 
                                item
                                    .replace("=","")
                                    .replace("{","")
                                    .replace("}","");

                            let trimmed = replaced.trim().to_string();
                            let vec_pair = trimmed.split(":").map(|s| s.to_string()).collect::<Vec<_>>();

                            items.push(
                                format!(r#"("{}", {})"#, vec_pair[0].clone(), vec_pair[1].clone())
                            )
                        }

                        let mut new_line = false;
                        if node_data.find("\n").is_some() {
                            new_line = true;
                        }
                        format!(
                            ": std::collections::HashMap<&str, _> = [{}].iter().cloned().collect(){}",
                            items.join(","),
                            if new_line { ";\n" } else { "" })
                    },
                    TokenKind::Operator(Operator::Add) => "+".to_string(),
                    TokenKind::Operator(Operator::Minus) => "-".to_string(),
                    TokenKind::Operator(Operator::Multiply) => "*".to_string(),
                    TokenKind::Operator(Operator::Divide) => "/".to_string(),
                    TokenKind::Operator(Operator::Mod) => "%".to_string(),
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
                    TokenKind::Lexeme(Lexeme::String) => handle_new_line(&mut new_line, node_data),
                    TokenKind::Keyword(Keyword::Return) => {
                        new_line = false;
                        "".to_string()
                    },
                    _ => "".to_string()
                }
            );
        }
        syntax.push(transpile(node));
    }
    syntax.join("")
}

fn handle_new_line(new_line: &mut bool, data: &str) -> String {
    if *new_line {
        str::replace(data, "\n", ";\n")
    } else {
        if data.find("\n").is_some() {
            *new_line = true;
        }
        data.to_string()
    }
}
