use regex::{ Regex, Captures };
use crate::keyword::Keyword;
use crate::token::{Token, TokenKind};
use crate::ast::Node;

pub fn recursive_parse<'a>(syntax: &'a str, tree: &mut Node, line_number: i32) -> 
    Result<bool, String> 
{
        let mut result = parse_to_token(syntax, line_number);
        let mut new_line_number = line_number;

        if result.is_none() {
            result = parse_as(syntax, line_number);
        }

        if result.is_none() {
            result = parse_derive(syntax, line_number);
        }

        if result.is_none() {
            result = parse_identifier_end(syntax, line_number);
        }

        if result.is_none() {
            result = parse_identifier(syntax, line_number);
        }

        if result.is_none() {
            result = parse_line_comment(syntax, line_number);
        }

        if result.is_none() {
            result = parse_open_parens(syntax, line_number);
        }

        if result.is_none() {
            result = parse_close_parens(syntax, line_number);
        }

        if result.is_none() {
            result = parse_dot(syntax, line_number);
        }

        if result.is_none() {
            result = parse_comma(syntax, line_number);
        }

        if result.is_none() {
            result = parse_block_comment(syntax, line_number);
            if let Some(result_parsed) = &result {
                let newlines = match_newlines((result_parsed.1).0);
                new_line_number = new_line_number + newlines.len() as i32;
            }
        }

        if result.is_none() {
            result = parse_whitespace(syntax, line_number);
        }

        if result.is_none() {
            result = parse_newline(syntax, line_number);
            new_line_number = new_line_number + 1;
        }

        if let Some(result_parsed) = result {
            tree.children.push(Node {
                token: result_parsed.0,
                children: vec![],
                data: Some((result_parsed.1).0.to_string())
            });
            recursive_parse((result_parsed.1).1, tree, new_line_number);
        } else {
            return Err(format!("pattern not recognize {}", syntax))
        }

    Ok(true)
}

macro_rules! parse_capture {
    ($syntax:expr, $RE:tt, $token_kind:tt, $line_number:expr, $eos:tt) => {
        if let Some(caps) = $RE.captures($syntax) {
            Some((
                Token::new($token_kind, $line_number, $eos),
                (
                    caps.get(1).map_or("", |m| m.as_str()),
                    caps.get(2).map_or("", |m| m.as_str()),
                )
            ))
        } else {
            None
        }
    }
}

fn parse_open_parens<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(\\()(?s)(.*)$").unwrap();
    }
    let token_kind = TokenKind::OpenParen;
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_close_parens<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(\\))(?s)(.*)$").unwrap();
    }
    let token_kind = TokenKind::CloseParen;
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_dot<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(\\.)(?s)(.*)$").unwrap();
    }
    let token_kind = TokenKind::Dot;
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_comma<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(,)(?s)(.*)$").unwrap();
    }
    let token_kind = TokenKind::Comma;
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_line_comment<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(//.*)(?s)(.*)$").unwrap();
    }
    let token_kind = TokenKind::LineComment;
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_block_comment<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(?s)(/\\*.*\\*/)(.*)$").unwrap();
    }
    let token_kind = TokenKind::BlockComment;
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn match_newlines<'a>(syntax: &'a str) -> Vec<Captures> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\n").unwrap();
    }
    RE.captures_iter(syntax).collect()
}

fn parse_identifier<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^([A-Za-z_0-9]+)(?s)(\\s|\\(|.*)$").unwrap();
    }
    let token_kind = TokenKind::Identifier;
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_identifier_end<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^([A-Za-z_0-9]+\\n)(?s)(\\s|\\(|.*)$").unwrap();
    }
    let token_kind = TokenKind::Identifier;
    parse_capture!(syntax, RE, token_kind, line_number, true)
}

fn parse_whitespace<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^([[:blank:]])(?s)(.*)$").unwrap();
    }
    let token_kind = TokenKind::Whitespace;
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_newline<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(\\n)(?s)(.*)$").unwrap();
    }
    let token_kind = TokenKind::NewLine;
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_as<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(\\sas)(?s)(.*)$").unwrap();
    }
    let token_kind = TokenKind::Keyword(Keyword::As);
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_derive<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(derive\\(.*\\))(?s)(.*)$").unwrap();
    }

    if let Some(caps) = RE.captures(syntax) {
        Some(
                (
                    Token::new(TokenKind::Keyword(Keyword::Derive), line_number, false),
                    (
                        caps.get(1).map_or("", |m| m.as_str()),
                        caps.get(2).map_or("", |m| m.as_str()),
                    )
                )
            )
    } else {
        None
    }
}

fn parse_to_token<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {

    for parsing in [
        (
            "module",
            Token::new(TokenKind::Keyword(Keyword::Module), line_number, false),
        ),
        (
            "do",
            Token::new(TokenKind::Keyword(Keyword::Do), line_number, false),
        ),
        (
            "end",
            Token::new(TokenKind::Keyword(Keyword::End), line_number, false),
        ),
        (
            "trait",
            Token::new(TokenKind::Keyword(Keyword::Trait), line_number, false),
        ),
        (
            "match",
            Token::new(TokenKind::Keyword(Keyword::Match), line_number, false),
        ),
        (
            "enum",
            Token::new(TokenKind::Keyword(Keyword::Enum), line_number, false),
        ),
        (
            "use",
            Token::new(TokenKind::Keyword(Keyword::Use), line_number, false),
        ),
        (
            "extern crate",
            Token::new(TokenKind::Keyword(Keyword::ExternCrate), line_number, false),
        ),
        (
            "struct",
            Token::new(TokenKind::Keyword(Keyword::Struct), line_number, false),
        ),
        (
            "public",
            Token::new(TokenKind::Keyword(Keyword::Public), line_number, false),
        ),
        (
            "implements",
            Token::new(TokenKind::Keyword(Keyword::Implements), line_number, false),
        ),
        (
            "inherits",
            Token::new(TokenKind::Keyword(Keyword::Inherits), line_number, false),
        ),
        (
            "if",
            Token::new(TokenKind::Keyword(Keyword::If), line_number, false),
        ),
        (
            "then",
            Token::new(TokenKind::Keyword(Keyword::Then), line_number, false),
        ),
        (
            "else",
            Token::new(TokenKind::Keyword(Keyword::Else), line_number, false),
        ),
        (
            "for",
            Token::new(TokenKind::Keyword(Keyword::For), line_number, false),
        ),
        (
            "in",
            Token::new(TokenKind::Keyword(Keyword::In), line_number, false),
        ),
        (
            "let",
            Token::new(TokenKind::Keyword(Keyword::Let), line_number, false),
        ),
        (
            "optional",
            Token::new(TokenKind::Keyword(Keyword::Optional), line_number, false),
        ),
        (
            "equal",
            Token::new(TokenKind::Keyword(Keyword::Equal), line_number, false),
        ),
        (
            "function",
            Token::new(TokenKind::Keyword(Keyword::Function), line_number, false),
        ),
        (
            "mutable",
            Token::new(TokenKind::Keyword(Keyword::Mutable), line_number, false),
        ),
        (
            "borrow",
            Token::new(TokenKind::Keyword(Keyword::Borrow), line_number, false),
        ),
        (
            "own",
            Token::new(TokenKind::Keyword(Keyword::Own), line_number, false),
        ),
        (
            "return",
            Token::new(TokenKind::Keyword(Keyword::Return), line_number, false),
        ),
    ]
    .into_iter()
    {
        let (pattern, token) = parsing;
        let full_pattern = format!("^({})(?s)(.*)$", pattern);

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
