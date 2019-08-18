#[macro_use] extern crate regex;
#[macro_use] extern crate lazy_static;

use regex::Error as RegexError;
use regex::Regex;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::str;

mod keyword;
mod token;

use keyword::Keyword;
use token::{Token, TokenKind};

#[test]
fn test() {
    let file_handler = File::open("examples/test1.am")
        .expect("Something went wrong reading the syntax file");

    let file = BufReader::new(&file_handler);
    for (num, line) in file.lines().enumerate() {
        let contents = line.expect(&format!("Something wrong reading line: {}", num));
        recursive_parse(&contents, num as i32 + 1);
        break;
    }
    
    assert_eq!(1 + 1, 5);
}

fn recursive_parse<'a>(syntax: &'a str, line_number: i32) -> &'a str {
    [
        ("module", Token::new(TokenKind::keyword(Keyword::Module), line_number, 1)),
        ("do", Token::new(TokenKind::keyword(Keyword::Do),  line_number, 1))
    ]
    .iter()
    .fold(syntax, move |sub_syntax, (str_token, token)| {
        let result = 
            parse_to_token(
                str_token,
                *token,
                sub_syntax,
            );
        dbg!(&result);
        result.1
    })
}

fn parse_to_token<'a>(
    str_token: &'a str,
    token: Token,
    syntax: &'a str,
) -> ((&'a str, Token), &'a str) {
    let tuple_result = parse(str_token, syntax);
    ((tuple_result.0, token), tuple_result.1)
}

fn parse<'a>(pattern: &'a str, syntax: &'a str) -> (&'a str, &'a str) {
    let re = Regex::new(&format!("^({})(.*)$", pattern)).unwrap();
    
    if let Some(caps) = re.captures(syntax) {
        (
            caps.get(1).map_or("", |m| m.as_str()),
            caps.get(2).map_or("", |m| m.as_str()),
        )
    } else {
        (
            syntax,
            ""
        )
    }

}
