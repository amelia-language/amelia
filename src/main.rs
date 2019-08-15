#[macro_use]
extern crate nom;

use std::str;
use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, digit1, one_of, space1, space0, newline, not_line_ending},
    character::is_alphabetic,
    combinator::{cut, map, map_res, opt},
    error::{context, VerboseError, ErrorKind},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[test]
fn test() {
    let contents = fs::read_to_string("examples/test1.am")
        .expect("Something went wrong reading the syntax file");
    recursive_parse(&contents);
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
    Optional,
    Let,
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
    Or,
}

#[derive(Debug, Clone)]
enum BuiltInType {
    String,
    Int32,
    Int64,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Usize,
    Isize,
    Boolean,
    Array,
    HashMap,
    Tuple
}

#[derive(Debug, Clone)]
enum BuiltInValue {
    Int32(i32),
    Int64(i64),
    UInt32(u32),
    UInt64(u64),
    Float32(f32),
    Float64(f64),
    Usize(usize),
    Isize(isize),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone)]
enum BuiltIn {
    OpenParen(char),
    CloseParen(char),
    Op(Operator),
    LogOp(LogicalOperator),
    Type(BuiltInType),
    Value(BuiltInValue),
    KeyWord(KeyWord),
    NewLine,
    Assigns
}

#[derive(Debug, Clone)]
enum Lexer {
    BuiltIn(BuiltIn),
    Identifier(String),
}

fn recursive_parse<'a>(i: &'a str) {
    if i != "" {
        match parse(i) {
            Ok(parsed) => {
                println!("{:#?}", parsed.1);
                recursive_parse(parsed.0)
            }
            Err(error) => {
                println!("{:#?}", error);
            }
        }
    }
}

fn parse<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    alt((
        parse_builtin,
        map(preceded(space1, alphanumeric1), |lexeme: &str| {
            Lexer::Identifier(lexeme.to_string())
        }),
        map(preceded(space0, newline), |_| {
            Lexer::BuiltIn(BuiltIn::NewLine)
        }),
        map(preceded(space1, tag("equal")), |_| {
            Lexer::BuiltIn(BuiltIn::Op(Operator::Equal))
        }),
        map(preceded(space1, tag("=")), |_| {
            Lexer::BuiltIn(BuiltIn::Assigns)
        }),
    ))(i)
}

fn parse_builtin<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    alt((
        parse_builtin_op,
        parse_builtin_log_op,
        parse_bool,
        parse_keyword,
        parse_builtin_types,
        parse_builtin_num,
        parse_string
    ))(i)
}

fn parse_builtin_op<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    let (i, t) = one_of("+-*/")(i)?;

    Ok((
        i,
        match t {
            '+' => Lexer::BuiltIn(BuiltIn::Op(Operator::Plus)),
            '-' => Lexer::BuiltIn(BuiltIn::Op(Operator::Minus)),
            '*' => Lexer::BuiltIn(BuiltIn::Op(Operator::Times)),
            '/' => Lexer::BuiltIn(BuiltIn::Op(Operator::Divide)),
            _ => unreachable!(),
        },
    ))
}

fn parse_builtin_log_op<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    alt((
        map(tag("not"), |_| {
            Lexer::BuiltIn(BuiltIn::LogOp(LogicalOperator::Not))
        }),
        map(tag("and"), |_| {
            Lexer::BuiltIn(BuiltIn::LogOp(LogicalOperator::And))
        }),
        map(tag("or"), |_| {
            Lexer::BuiltIn(BuiltIn::LogOp(LogicalOperator::Or))
        }),
    ))(i)
}

fn parse_bool<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    alt((
        map(tag("true"), |_| Lexer::BuiltIn(BuiltIn::Value(BuiltInValue::Boolean(true)))),
        map(tag("false"), |_| Lexer::BuiltIn(BuiltIn::Value(BuiltInValue::Boolean(false)))),
    ))(i)
}

fn parse_keyword<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    alt((
        map(preceded(space1, tag("if")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::If))
        }),
        map(preceded(space1, tag("then")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Then))
        }),
        map(preceded(space1, tag("else")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Else))
        }),
        map(preceded(space1, tag("for")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::For))
        }),
        map(preceded(space1, tag("do")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Do))
        }),
        map(preceded(space0, tag("end")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::End))
        }),
        map(preceded(space0, tag("public")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Public))
        }),
        map(preceded(space0, tag("struct")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Struct))
        }),
        map(preceded(space0, tag("implements")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Implements))
        }),
        map(preceded(space1, tag("inherits")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Inherits))
        }),
        map(preceded(space1, tag("in")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::In))
        }),
        map(preceded(space1, tag("as")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::As))
        }),
        map(preceded(space1, tag("equal")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Equal))
        }),
        map(preceded(space0, tag("function")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Function))
        }),
        map(preceded(space1, tag("mutable")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Mutable))
        }),
        map(preceded(space1, tag("borrow")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Borrow))
        }),
        map(preceded(space1, tag("own")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Own))
        }),
        map(preceded(space1, tag("optional")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Optional))
        }),
        map(preceded(space0, tag("let")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Let))
        }),
    ))(i)
}

macro_rules! parse_type {
    ($tag_fn:expr, $builtin_type:expr) => {
        map(preceded(space1, $tag_fn), |_| {
            Lexer::BuiltIn(BuiltIn::Type($builtin_type))
        })
    };
}

fn parse_builtin_types<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    alt((
        parse_type!(tag("String"), BuiltInType::String),
        parse_type!(tag("Int32"), BuiltInType::Int32),
        parse_type!(tag("Int64"), BuiltInType::Int64),
        parse_type!(tag("UInt32"), BuiltInType::UInt32),
        parse_type!(tag("UInt64"), BuiltInType::UInt64),
        parse_type!(tag("Float32"), BuiltInType::Float32),
        parse_type!(tag("Float64"), BuiltInType::Float64),
        parse_type!(tag("USize"), BuiltInType::Usize),
        parse_type!(tag("ISize"), BuiltInType::Isize),
        parse_type!(tag("Boolean"), BuiltInType::Boolean),
        parse_type!(tag("Array"), BuiltInType::Array),
        parse_type!(tag("HashMap"), BuiltInType::HashMap),
        parse_type!(tag("Tuple"), BuiltInType::Tuple),
    ))(i)
}

macro_rules! parse_num {
    ($num_type:ty, $built_in_emu_type:expr) => {
        map_res(digit1, |digit_str: &str| {
            digit_str.parse::<$num_type>().map(|digit| {
                Lexer::BuiltIn(BuiltIn::Value($built_in_emu_type(digit)))
            })
        })
    };
}

fn parse_builtin_num<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
  alt((
    parse_num!(i32, BuiltInValue::Int32),
    parse_num!(i64, BuiltInValue::Int64),
    parse_num!(u32, BuiltInValue::UInt32),
    parse_num!(u64, BuiltInValue::UInt64),
    parse_num!(f32, BuiltInValue::Float32),
    parse_num!(f64, BuiltInValue::Float64),
    parse_num!(usize, BuiltInValue::Usize),
    parse_num!(isize, BuiltInValue::Isize),
  ))(i)
}

fn parse_string<'a>(i: &'a [u8]) -> IResult<&'a [u8], Lexer, (&'a [u8], ErrorKind)> {
  context("string",
    map(parse_string_bytes, |lexeme: &[u8]| {
        Lexer::BuiltIn(
            BuiltIn::Value(
                BuiltInValue::String(
                    String::from_utf8_lossy(lexeme).to_string()
                )
            )
        )
    })
  )(i)
}


fn parse_identifier<'a>(i: &'a [u8]) -> IResult<&'a [u8], Lexer, (&'a [u8], ErrorKind)> {
    context("identifier",
        map(parse_ident_bytes, |lexeme: &[u8]| {
            Lexer::Identifier(String::from_utf8_lossy(lexeme).to_string())
        })
    )(i)
}

named!(parse_ident_bytes, re_bytes_match!(r"^_?[A-Za-z][0-9A-Z_a-z-]*"));
named!(parse_string_bytes, re_bytes_match!(r#""\w.*""#));