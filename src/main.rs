#[macro_use]
extern crate regex;

use std::str;
use std::fs;

mod token;
mod keyword;

#[test]
fn test() {
    let contents = fs::read_to_string("examples/test1.am")
        .expect("Something went wrong reading the syntax file");
    recursive_parse(&contents);
    assert_eq!(1 + 1, 5);
}

fn recursive_parse<'a>(i: &'a str) -> &'a str {
    i
}
