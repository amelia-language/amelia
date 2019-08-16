#[macro_use]
extern crate nom;

use std::str;
use std::fs;

use nom::{
    InputTakeAtPosition, AsChar,
    branch::alt,
    bytes::complete::{tag, escaped, take_while},
    character::is_alphanumeric,
    character::complete::{alphanumeric1, 
                          alpha1,
                          char,
                          digit1,
                          one_of,
                          space1,
                          space0,
                          newline,
                          not_line_ending,
                          multispace0},
    combinator::{cut, map, map_res},
    error::{context, VerboseError, ErrorKind, ParseError},
    sequence::{preceded, terminated, delimited},
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
    Op(Operator),
    LogOp(LogicalOperator),
    Type(BuiltInType),
    Value(BuiltInValue),
    KeyWord(KeyWord),
    NewLine,
    Assigns,
    Comments
}

#[derive(Debug, Clone)]
enum Lexer {
    BuiltIn(BuiltIn),
    Identifier(String),
    MacroInvk(String)
}

fn recursive_parse<'a>(i: &'a str) {
    if i != "" {
        match parse(i) {
            Ok(parsed) => {
                println!("parsed1: {:#?}", parsed.1);
                recursive_parse(parsed.0)
            }
            Err(error) => {
                println!("{:#?}", error);
            }
        }
    }
}

fn parse<'a>(i: &'a str) -> IResult<&'a str, Lexer, (&'a str, ErrorKind)> {
    alt((
        map(preceded(space1, tag("equal")), |_| {
            Lexer::BuiltIn(BuiltIn::Op(Operator::Equal))
        }),
        map(preceded(space1, tag("=")), |_| {
            Lexer::BuiltIn(BuiltIn::Assigns)
        }),
        map(newline, |_| {
            Lexer::BuiltIn(BuiltIn::NewLine)
        }),
        parse_builtin,
    ))(i)
}

fn parse_builtin<'a>(i: &'a str) -> IResult<&'a str, Lexer, (&'a str, ErrorKind)> {
    alt((
        parse_comments,
        parse_builtin_op,
        parse_builtin_log_op,
        parse_bool,
        parse_builtin_keyword,
        parse_builtin_types,
        parse_builtin_num,
        parse_string,
        parse_macro_invokation,
        parse_builtin_identifier,
    ))(i)
}

fn parse_builtin_op<'a>(i: &'a str) -> IResult<&'a str, Lexer, (&'a str, ErrorKind)> {
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

fn parse_builtin_log_op<'a>(i: &'a str) -> IResult<&'a str, Lexer, (&'a str, ErrorKind)> {
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

fn parse_bool<'a>(i: &'a str) -> IResult<&'a str, Lexer, (&'a str, ErrorKind)> {
    alt((
        map(tag("true"), |_| Lexer::BuiltIn(BuiltIn::Value(BuiltInValue::Boolean(true)))),
        map(tag("false"), |_| Lexer::BuiltIn(BuiltIn::Value(BuiltInValue::Boolean(false)))),
    ))(i)
}

macro_rules! parse_keyword {
    ($tag_fn:expr, $builtin_type:expr) => {
        map(delimited(multispace0, $tag_fn, space1), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord($builtin_type))
        })
    };
}

fn parse_builtin_keyword<'a>(i: &'a str) -> IResult<&'a str, Lexer, (&'a str, ErrorKind)> {
    alt((
        parse_keyword!(tag("if"), KeyWord::If),
        parse_keyword!(tag("then"), KeyWord::Then),
        parse_keyword!(tag("else"), KeyWord::Else),
        parse_keyword!(tag("for"), KeyWord::For),
        parse_keyword!(tag("do"), KeyWord::Do),
        parse_keyword!(tag("end"), KeyWord::End),
        parse_keyword!(tag("struct"), KeyWord::Struct),
        parse_keyword!(tag("public"), KeyWord::Public),
        parse_keyword!(tag("implements"), KeyWord::Implements),
        parse_keyword!(tag("inherits"), KeyWord::Inherits),
        parse_keyword!(tag("in"), KeyWord::In),
        parse_keyword!(tag("as"), KeyWord::As),
        parse_keyword!(tag("equal"), KeyWord::Equal),
        parse_keyword!(tag("function"), KeyWord::Function),
        parse_keyword!(tag("mutable"), KeyWord::Mutable),
        parse_keyword!(tag("borrow"), KeyWord::Borrow),
        parse_keyword!(tag("own"), KeyWord::Own),
        parse_keyword!(tag("optional"), KeyWord::Optional),
        parse_keyword!(tag("let"), KeyWord::Let),
        map(preceded(space0, terminated(tag("do"), newline)), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Do))
        }),
        map(preceded(space0, terminated(tag("end"), newline)), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::End))
        })

    ))(i)
}

macro_rules! parse_type {
    ($tag_fn:expr, $builtin_type:expr) => {
        map(delimited(space0, $tag_fn, alt((tag(","), tag(")"), tag("\n"), space1)) ), |_| {
            Lexer::BuiltIn(BuiltIn::Type($builtin_type))
        })
    };
}

fn parse_builtin_types<'a>(i: &'a str) -> IResult<&'a str, Lexer, (&'a str, ErrorKind)> {
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

fn parse_builtin_num<'a>(i: &'a str) -> IResult<&'a str, Lexer, (&'a str, ErrorKind)> {
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

fn parse_str<'a>(i: &'a str) -> IResult<&'a str, &'a str, (&'a str, ErrorKind)> {
  escaped(alphanumeric1, '\\', one_of("\"n\\"))(i)
}

fn parse_string<'a>(i: &'a str) -> IResult<&'a str, Lexer, (&'a str, ErrorKind)> {
  context("string",
    map(
        preceded(
        char('\"'),
        cut(terminated(
            parse_str,
            char('\"')
        ))),
        |lexeme: &str| {
            Lexer::BuiltIn(
                BuiltIn::Value(
                    BuiltInValue::String(
                        lexeme.to_string()
                    )
                )
            )
        })
  )(i)
}

fn parse_comments<'a>(i: &'a str) -> IResult<&'a str, Lexer, (&'a str, ErrorKind)> {
  context("comments",
    map(
        preceded(tag("// "), not_line_ending), |_| Lexer::BuiltIn(BuiltIn::Comments))
  )(i)
}

fn parse_builtin_identifier<'a>(i: &'a str) -> IResult<&'a str, Lexer, (&'a str, ErrorKind)> {
  context("identifier",
    map(terminated(take_while(is_alphanum_or_underscore), 
        alt((space1, tag("("), tag(","), tag(")")))), |lexeme: &str| {
            Lexer::Identifier(lexeme.to_string())
        }
    )
  )(i)
}

fn parse_macro_invokation<'a>(i: &'a str) -> IResult<&'a str, Lexer, (&'a str, ErrorKind)> {
  context("macro_invokation",
    map(terminated(take_while(is_alphanum_or_underscore), tag("!(")), |lexeme: &str| {
            Lexer::MacroInvk(lexeme.to_string())
        }
    )
  )(i)
}

pub fn is_alphanum_or_underscore(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}