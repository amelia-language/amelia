#[macro_use]
extern crate nom;

use std::str;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, digit1, one_of, space0},
    character::is_alphabetic,
    combinator::{cut, map, map_res, opt},
    error::{context, VerboseError},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[test]
fn test() {
    let syntax = "public struct Animal do \n public sound as String \n public age as Int32 \n end";
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
    Or,
}

#[derive(Debug, Clone)]
enum BuiltIn {
    Paren(char),
    Op(Operator),
    LogOp(LogicalOperator),
    Int32(i32),
    Int64(i64),
    UInt32(u32),
    UInt64(u64),
    Float32(f32),
    Float64(f64),
    Usize(usize),
    Isize(isize),
    String(String),
    Type(String),
    Boolean(bool),
    KeyWord(KeyWord),
    NewLine,
}

#[derive(Debug, Clone)]
enum Lexer {
    BuiltIn(BuiltIn),
    Identifier(String),
}

fn recursive_parse<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    match parse(i) {
        Ok(parsed) => {
            println!("{:#?}", parsed.1);
            recursive_parse(parsed.0)
        }
        Err(error) => {
            println!("{:#?}", error);
            Err(error)
        }
    }
}

fn parse<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    alt((
        parse_builtin,
        map(preceded(space0, alphanumeric1), |lexeme: &str| {
            Lexer::Identifier(lexeme.to_string())
        }),
        map(preceded(space0, tag("\n")), |_| {
            Lexer::BuiltIn(BuiltIn::NewLine)
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
        map(tag("true"), |_| Lexer::BuiltIn(BuiltIn::Boolean(true))),
        map(tag("false"), |_| Lexer::BuiltIn(BuiltIn::Boolean(false))),
    ))(i)
}

fn parse_keyword<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    alt((
        map(preceded(space0, tag("if")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::If))
        }),
        map(preceded(space0, tag("then")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Then))
        }),
        map(preceded(space0, tag("else")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Else))
        }),
        map(preceded(space0, tag("for")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::For))
        }),
        map(preceded(space0, tag("in")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::In))
        }),
        map(preceded(space0, tag("do")), |_| {
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
        map(preceded(space0, tag("inherits")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Inherits))
        }),
        map(preceded(space0, tag("as")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::As))
        }),
        map(preceded(space0, tag("equal")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Equal))
        }),
        map(preceded(space0, tag("function")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Function))
        }),
        map(preceded(space0, tag("mutable")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Mutable))
        }),
        map(preceded(space0, tag("borrow")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Borrow))
        }),
        map(preceded(space0, tag("own")), |_| {
            Lexer::BuiltIn(BuiltIn::KeyWord(KeyWord::Own))
        }),
    ))(i)
}


fn parse_type<'a, F>(
    tag_fn: F,
) -> impl Fn(&'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>>
where
    F: Fn(&'a str) -> IResult<&'a str, &'a str, VerboseError<&'a str>>,
{
    map(preceded(space0, tag_fn), |lexeme: &str| {
        Lexer::BuiltIn(BuiltIn::Type(lexeme.to_string()))
    })
}

fn parse_builtin_types<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
    alt((
        parse_type(tag("String")),
        parse_type(tag("Int32")),
        parse_type(tag("Int64")),
        parse_type(tag("UInt32")),
        parse_type(tag("UInt64")),
        parse_type(tag("Float32")),
        parse_type(tag("Float64")),
        parse_type(tag("USize")),
        parse_type(tag("ISize")),
    ))(i)
}

macro_rules! parse_num {
    ($num_type:ty, $built_in_emu_type:expr) => {
        map_res(digit1, |digit_str: &str| {
            digit_str.parse::<$num_type>().map(|digit| {
                Lexer::BuiltIn($built_in_emu_type(digit))
            })
        })
    };
}

fn parse_builtin_num<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
  alt((
    parse_num!(i32, BuiltIn::Int32),
    parse_num!(i64, BuiltIn::Int64),
    parse_num!(u32, BuiltIn::UInt32),
    parse_num!(u64, BuiltIn::UInt64),
    parse_num!(f32, BuiltIn::Float32),
    parse_num!(f64, BuiltIn::Float64),
    parse_num!(usize, BuiltIn::Usize),
    parse_num!(isize, BuiltIn::Isize),
  ))(i)
}

fn parse_string<'a>(i: &'a str) -> IResult<&'a str, Lexer, VerboseError<&'a str>> {
  context("string",
    map(
    preceded(
      char('\"'),
      cut(terminated(
          alphanumeric1,
          char('\"')
    ))), |text: &str| Lexer::BuiltIn(BuiltIn::String(text.to_string())))
  )(i)
}