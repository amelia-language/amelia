#[macro_use]
extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str;
use std::rc::Rc;

mod keyword;
mod token;
mod transpiler;
mod ast;
mod parser;

use ast::Node;
use transpiler::transpile;
use token::{Token, TokenKind};

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
    parser::recursive_parse(&contents, &mut tree, 1);
    dbg!(transpile(tree));
    assert_eq!(1 + 1, 5);
}

