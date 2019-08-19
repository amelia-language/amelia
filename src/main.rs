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

mod keyword;
mod token;

use keyword::Keyword;
use token::{Token, TokenKind};

#[test]
fn test() {
    let file_handler =
        File::open("examples/test1.am").expect("Something went wrong reading the syntax file");

    let file = BufReader::new(&file_handler);
    for (num, line) in file.lines().enumerate() {
        let contents = line.expect(&format!("Something wrong reading line: {}", num));
        recursive_parse(&contents, num as i32 + 1);
    }
    assert_eq!(1 + 1, 5);
}

fn recursive_parse<'a>(syntax: &'a str, line_number: i32) -> Result<bool, String> {
    let mut result = parse_to_token(syntax, line_number);
    if result.is_none() {
        result = parse_identifier(syntax, line_number);
    }

    if let Some(result_parsed) = result {
        recursive_parse((result_parsed.1).1, line_number);
    } else {
        return Err(format!("pattern not recognize {}", syntax))
    }

    Ok(true)
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
