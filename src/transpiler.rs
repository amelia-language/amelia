
use crate::ast::Node;
use crate::token::TokenKind;
use crate::keyword::Keyword;

pub fn transpile(ast: Node) -> String {
    let mut syntax = vec![];
    for node in ast.children {
        syntax.push(
            match node.token.kind {
                TokenKind::keyword(Keyword::Module) => "pub",
                _ => "other"
            }
        );
    }
    syntax.join("")
}