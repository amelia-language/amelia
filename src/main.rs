#[macro_use]
extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::fs;
use std::fs::File;
use std::io::Write;

mod keyword;
mod token;
mod transpiler;
mod ast;
mod parser;
mod lexeme;
mod block_keyword;

use ast::Node;
use transpiler::transpile;
use token::{Token, TokenKind};
use block_keyword::DO;

#[test]
fn test() {
    let contents = fs::read_to_string("examples/test1.am")
        .expect("Something went wrong reading the syntax file");

    let mut tree = 
        Node 
            {
                token: Token::new(TokenKind::Root, 0, false), 
                children: vec![], 
                data: None 
            };
    parser::complete_parse(&contents, &mut tree, 1, DO);
    let mut file = File::create("examples/testrs")
        .expect("Someting went wrong creating the file");
    file.write_all(transpile(tree).as_bytes());
}

