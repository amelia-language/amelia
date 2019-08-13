#[macro_use]
extern crate nom;

use std::str;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, space0, one_of},
    character::is_alphabetic,
    combinator::{cut, map, map_res, opt},
    error::{context, VerboseError},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[test]
fn test() {
    let syntax = "public struct Animal do \n public sound as String \n public age as Int \n end";
    recursive_parse(syntax);
    assert_eq!(1 + 1, 5);
}

#[derive(Debug, Clone)]
enum KeyWord {
    If,
    Then,
    Else,
    For,
    Do,
    End,
    Public,
    Struct,
    Implements,
    Inherits,
    As,
    Equal,
    Function,
    Mutable,
    Borrow,
    Own,
    In,
}

#[derive(Debug, Clone)]
enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
}

#[derive(Debug, Clone)]
enum LogicalOperator {
    Not,
    And,
    Or
}

#[derive(Debug, Clone)]
enum BuiltIn {
    Paren(char),
    Op(Operator),
    LogOp(LogicalOperator),
    Int(u64),
    Float(f64),
    Boolean(bool),
    KeyWord(KeyWord),
    NewLine
}

#[derive(Debug, Clone)]
enum Lexer {
    BuiltIn(BuiltIn),
    Identifier
}

fn recursive_parse<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    match parse(i) {
        Ok(parsed) => {
            println!("{:#?}", parsed.1);
            recursive_parse(parsed.0)
        },
        Err(error) => {
            println!("{:#?}", error);
            Err(error)
        } 
    }
}

fn parse<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    alt((parse_builtin, 
         map(preceded(space0, alpha1), |_| Lexer::Identifier),
         map(preceded(space0, tag("\n")), |_| Lexer::BuiltIn(BuiltIn::NewLine))
    ))(i)
}

fn parse_builtin<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    alt((parse_builtin_op, parse_builtin_log_op, parse_bool, parse_keyword))(i)
}

fn parse_builtin_op<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    let (i, t) = one_of("+-*/=")(i)?;

    Ok((
        i,
        match t {
            '+' => Lexer::BuiltIn(BuiltIn::Op(Operator::Plus)),
            '-' => Lexer::BuiltIn(BuiltIn::Op(Operator::Minus)),
            '*' => Lexer::BuiltIn(BuiltIn::Op(Operator::Times)),
            '/' => Lexer::BuiltIn(BuiltIn::Op(Operator::Divide)),
            '=' => Lexer::BuiltIn(BuiltIn::Op(Operator::Equal)),
            _ => unreachable!(),
        },
    ))
}

fn parse_builtin_log_op<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    alt((
        map(tag("not"), |_| Lexer::BuiltIn(BuiltIn::LogOp(LogicalOperator::Not))),
        map(tag("and"), |_| Lexer::BuiltIn(BuiltIn::LogOp(LogicalOperator::And))),
        map(tag("or"), |_|  Lexer::BuiltIn(BuiltIn::LogOp(LogicalOperator::Or))),
    ))(i)
}

fn parse_bool<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    alt((
        map(tag("true"), |_|  Lexer::BuiltIn(BuiltIn::Boolean(true))),
        map(tag("false"), |_| Lexer::BuiltIn(BuiltIn::Boolean(false))),
    ))(i)
}

fn parse_keyword<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    alt((
        map(preceded(space0, tag("if")), |_|   Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::If))),
        map(preceded(space0, tag("then")), |_| Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Then))),
        map(preceded(space0, tag("else")), |_| Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Else))),
        map(preceded(space0, tag("for")), |_|  Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::For))),
        map(preceded(space0, tag("in")), |_|   Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::In))),
        map(preceded(space0, tag("do")), |_|   Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Do))),
        map(preceded(space0, tag("end")), |_|  Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::End))),
        map(preceded(space0, tag("public")), |_| Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Public))),
        map(preceded(space0, tag("struct")), |_| Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Struct))),
        map(preceded(space0, tag("implements")), |_| Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Implements))),
        map(preceded(space0, tag("inherits")), |_| Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Inherits))),
        map(preceded(space0, tag("as")), |_| Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::As))),
        map(preceded(space0, tag("equal")), |_| Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Equal))),
        map(preceded(space0, tag("function")), |_| Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Function))),
        map(preceded(space0, tag("mutable")), |_| Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Mutable))),
        map(preceded(space0, tag("borrow")), |_| Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Borrow))),
        map(preceded(space0, tag("own")), |_| Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Own))),
    ))(i)
}
