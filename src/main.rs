#[macro_use]
extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Error as RegexError;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str;
use std::rc::Rc;

mod keyword;
mod token;

use keyword::Keyword;
use token::{Token, TokenKind};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    token: Token,
    children: Vec<Node>,
    data: Option<String>,
}

#[test]
fn test() {
    let contents = fs::read_to_string("examples/test1.am")
        .expect("Something went wrong reading the syntax file");

    let mut tree = 
        Node 
            {
                token: Token::new(TokenKind::Root, 0), 
                children: vec![], 
                data: None 
            };
    recursive_parse(&contents, &mut tree, 1);
    dbg!(tree);
    assert_eq!(1 + 1, 5);
}

fn recursive_parse<'a>(syntax: &'a str, tree: &mut Node, line_number: i32) -> 
    Result<bool, String> 
{
        let mut result = parse_to_token(syntax, line_number);
        if result.is_none() {
            result = parse_identifier(syntax, line_number);
        }

        if result.is_none() {
            result = parse_comment(syntax, line_number);
        }

        dbg!(&result);

        if let Some(result_parsed) = result {
            tree.children.push(Node {
                token: result_parsed.0,
                children: vec![],
                data: Some((result_parsed.1).0.to_string())
            });
            recursive_parse((result_parsed.1).1, tree, line_number);
        } else {
            return Err(format!("pattern not recognize {}", syntax))
        }

    Ok(true)
}

fn parse_comment<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    let full_pattern = format!("^//.*$");
    parse(full_pattern, syntax)
        .map(|pattern| {
            (Token::new(TokenKind::LineComment, line_number), (pattern.0, pattern.1))
        })
}

fn parse_identifier<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    let full_pattern = format!("^([a-z_]+)(\\s.*)$");
    parse(full_pattern, syntax)
        .map(|pattern| {
            (Token::new(TokenKind::Identifier, line_number), (pattern.0, pattern.1))
        })
}

fn parse_to_token<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {

    for parsing in [
        (
            "module",
            Token::new(TokenKind::keyword(Keyword::Module), line_number),
        ),
        (
            "do",
            Token::new(TokenKind::keyword(Keyword::Do), line_number),
        ),
        ("\\s", Token::new(TokenKind::Whitespace, line_number)),
    ]
    .into_iter()
    {
        let (pattern, token) = parsing;
        let full_pattern = format!("^({})(.*)$", pattern);

        if let Some(parsed_result) = parse(full_pattern, syntax) {
            return Some((token.clone(), parsed_result));
        }
    }

    None
}

fn parse<'a>(pattern: String, syntax: &'a str) -> Option<(&'a str, &'a str)> {
    let re = Regex::new(&pattern).unwrap();
    if let Some(caps) = re.captures(syntax) {
        Some((
            caps.get(1).map_or("", |m| m.as_str()),
            caps.get(2).map_or("", |m| m.as_str()),
        ))
    } else {
        None
    }
}
