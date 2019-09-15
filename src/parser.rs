use regex::{ Regex, Captures };
use crate::keyword::Keyword;
use crate::token::{ Token, TokenKind, LiteralKind, Operator, Collection };
use crate::ast::Node;
use crate::lexeme::Lexeme;
use crate::block_keyword;

pub fn complete_parse<'a>(syntax: &'a str, tree: &mut Node, line_number: i32, begin_mark: &str) -> 
    Result<&'a str, String> 
{
    let mut full_code: &'a str = syntax;
    let mut new_line_number = line_number;
    let mut begin_group_scope = 0;
    let mut end_group_scope = 0;
    loop {
        let mut result = parse_hash_map(full_code, new_line_number);

        if result.is_none() {
            result = parse_to_token(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_macro_rules(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_macro(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_function_call(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_borrow(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_own(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_namespace_separator(full_code, new_line_number);
        }
            
        if result.is_none() {
            result = parse_operator(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_type(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_as(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_not_operator(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_derive(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_type_with_generics(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_string(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_array(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_tuple(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_identifier(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_line_comment(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_open_parens(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_close_parens(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_dot(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_comma(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_block_comment(full_code, new_line_number);
            if let Some(result_parsed) = &result {
                let newlines = match_newlines((result_parsed.1).0);
                new_line_number = new_line_number + newlines.len() as i32;
            }
        }

        if result.is_none() {
            result = parse_whitespace(full_code, new_line_number);
        }

        if result.is_none() {
            result = parse_newline(full_code, new_line_number);
            new_line_number = new_line_number + 1;
        }

        if let Some(result_parsed) = result {
            let token = result_parsed.0;
            let mut tree_with_children =
                Node {
                    token,
                    children: vec![],
                    data: Some((result_parsed.1).0.to_string())
                };
            full_code = (result_parsed.1).1;

            match tree_with_children.token.kind {
                TokenKind::Macro | TokenKind::MacroRules =>  {
                    full_code = parse_macro_body(full_code, &mut tree_with_children, line_number)?;
                },
                _ => {
                    if tree_with_children.token.kind == TokenKind::Keyword(Keyword::Function) || 
                        tree_with_children.token.kind == TokenKind::Keyword(Keyword::PublicFunction) {
                            let result_code = complete_parse(full_code, &mut tree_with_children, line_number, block_keyword::DO);
                            if let Ok(code) = result_code {
                                full_code = code;
                            }
                    }

                    if block_keyword::match_block_begin(&tree_with_children.token.kind, begin_mark) {
                        begin_group_scope += 1;
                    }
                    if block_keyword::match_block_end(&tree_with_children.token.kind, begin_mark) {
                        end_group_scope += 1;
                    }
                    if begin_group_scope == end_group_scope && begin_group_scope > 0 && end_group_scope > 0 {
                        return Ok(full_code)
                    }
                }
            }
            tree.children.push(tree_with_children);
        } else {
            return Err(format!("pattern not recognize {}", syntax))
        }

        if full_code == "" {
            return Ok(full_code)
        }
    }
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
        static ref RE: Regex = Regex::new("^(\\)\\n?)(?s)(.*)$").unwrap();
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
        static ref RE: Regex = Regex::new("^([A-Za-z_0-9]+\\n?)(?s)(\\s|\\(|.*)$").unwrap();
    }
    let token_kind = TokenKind::Identifier;
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_whitespace<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^([[:blank:]]+)(?s)(.*)$").unwrap();
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

fn parse_type_with_generics<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^([A-Za-z0-9]+<.*>\\n?)(?s)(.*)$").unwrap();
    }
    let token_kind = TokenKind::TypeWithGeneric;
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_macro<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^([A-Za-z_0-9]+!)(?s)(.*)$").unwrap();
    }
    let token_kind = TokenKind::Macro;
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_macro_rules<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(macro_rules!\\s[A-Za-z_0-9]+)(?s)(.*)$").unwrap();
    }
    let token_kind = TokenKind::MacroRules;
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_namespace_separator<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(::)(?s)(.*)$").unwrap();
    }
    let token_kind = TokenKind::NamespaceSeparator;
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_not_operator<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(not\\s)(?s)(.*)$").unwrap();
    }
    let token_kind = TokenKind::Not;
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_array<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(\\[.*,.*\\]\\n?)(?s)(\\s.*)$").unwrap();
    }
    let token_kind = TokenKind::Collection(Collection::Array);
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_tuple<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(\\(.*,.*\\)\\n?)(?s)(\\s.*)$").unwrap();
    }
    let token_kind = TokenKind::Collection(Collection::Tuple);
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_hash_map<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(=\\s*\\{.*,.*\\}\\n?)(?s)(\\s.*)$").unwrap();
    }
    let token_kind = TokenKind::Collection(Collection::HashMap);
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

fn parse_string<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^(?s)(".*?")(.*)$"#).unwrap();
    }
    let token_kind = TokenKind::Lexeme(Lexeme::String);
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_borrow<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(borrow\\s)(?s)(.*)$").unwrap();
    }
    let token_kind = TokenKind::Keyword(Keyword::Borrow);
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_own<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(own\\s)(?s)(.*)$").unwrap();
    }
    let token_kind = TokenKind::Keyword(Keyword::Own);
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_function_call<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^([A-Za-z_0-9]+\\s*\\()(?s)(.*)$").unwrap();
    }
    let token_kind = TokenKind::FunctionCall;
    parse_capture!(syntax, RE, token_kind, line_number, false)
}

fn parse_operator<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {

    for parsing in [
        (
            "\\+",
            Token::new(TokenKind::Operator(Operator::Add), line_number, false),
        ),
        (
            "-",
            Token::new(TokenKind::Operator(Operator::Minus), line_number, false),
        ),
        (
            "\\*",
            Token::new(TokenKind::Operator(Operator::Multiply), line_number, false),
        ),
        (
            "/",
            Token::new(TokenKind::Operator(Operator::Divide), line_number, false),
        ),
        (
            "%",
            Token::new(TokenKind::Operator(Operator::Mod), line_number, false),
        ),
    ]
    .into_iter()
    {
        let (pattern, token) = parsing;
        let full_pattern = format!("^({})(?s)(\\s.*)$", pattern);

        if let Some(parsed_result) = parse(full_pattern, syntax) {
            return Some((token.clone(), parsed_result));
        }
    }

    None
}

fn parse_type<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {

    for parsing in [
        (
            "Boolean",
            Token::new(TokenKind::Literal(LiteralKind::Boolean), line_number, false),
        ),
        (
            "String",
            Token::new(TokenKind::Literal(LiteralKind::String), line_number, false),
        ),
        (
            "Int32",
            Token::new(TokenKind::Literal(LiteralKind::Int32), line_number, false),
        ),
        (
            "Int64",
            Token::new(TokenKind::Literal(LiteralKind::Int64), line_number, false),
        ),
        (
            "UInt32",
            Token::new(TokenKind::Literal(LiteralKind::UInt32), line_number, false),
        ),
        (
            "UInt64",
            Token::new(TokenKind::Literal(LiteralKind::UInt64), line_number, false),
        ),
        (
            "Float64",
            Token::new(TokenKind::Literal(LiteralKind::Float64), line_number, false),
        ),
        (
            "Float32",
            Token::new(TokenKind::Literal(LiteralKind::Float32), line_number, false),
        ),
        (
            "Usize",
            Token::new(TokenKind::Literal(LiteralKind::Usize), line_number, false),
        ),
        (
            "Isize",
            Token::new(TokenKind::Literal(LiteralKind::Isize), line_number, false),
        ),
        (
            "Char",
            Token::new(TokenKind::Literal(LiteralKind::Char), line_number, false),
        ),
        (
            "Byte",
            Token::new(TokenKind::Literal(LiteralKind::Byte), line_number, false),
        ),
    ]
    .into_iter()
    {
        let (pattern, token) = parsing;
        let full_pattern = format!("^({}[,|>|)|\\n])(?s)(.*)$", pattern);

        if let Some(parsed_result) = parse(full_pattern, syntax) {
            return Some((token.clone(), parsed_result));
        }
    }

    None
}

fn parse_to_token<'a>(syntax: &'a str, line_number: i32) -> Option<(Token, (&'a str, &'a str))> {

    for parsing in [
        (
            "module",
            Token::new(TokenKind::Keyword(Keyword::Module), line_number, false),
        ),
        (
            "let",
            Token::new(TokenKind::Keyword(Keyword::Let), line_number, false),
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
            "public struct",
            Token::new(TokenKind::Keyword(Keyword::PublicStruct), line_number, false),
        ),
        (
            "public trait",
            Token::new(TokenKind::Keyword(Keyword::PublicTrait), line_number, false),
        ),
        (
            "public use",
            Token::new(TokenKind::Keyword(Keyword::PublicUse), line_number, false),
        ),
        (
            "public module",
            Token::new(TokenKind::Keyword(Keyword::PublicModule), line_number, false),
        ),
        (
            "function",
            Token::new(TokenKind::Keyword(Keyword::Function), line_number, false),
        ),
        (
            "public function",
            Token::new(TokenKind::Keyword(Keyword::PublicFunction), line_number, false),
        ),
        (
            "public enum",
            Token::new(TokenKind::Keyword(Keyword::PublicEnum), line_number, false),
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
            "equal",
            Token::new(TokenKind::Equal, line_number, false),
        ),
        (
            "not equal",
            Token::new(TokenKind::NotEqual, line_number, false),
        ),
        (
            "mutable",
            Token::new(TokenKind::Keyword(Keyword::Mutable), line_number, false),
        ),
        (
            "own",
            Token::new(TokenKind::Keyword(Keyword::Own), line_number, false),
        ),
        (
            "return",
            Token::new(TokenKind::Keyword(Keyword::Return), line_number, false),
        ),
        (
            "=",
            Token::new(TokenKind::Assign, line_number, false),
        ),
        (
            ":",
            Token::new(TokenKind::DoubleDot, line_number, false),
        ),
        (
            "=>",
            Token::new(TokenKind::FatArrow, line_number, false),
        ),
        (
            "\\|>",
            Token::new(TokenKind::PipeLine, line_number, false),
        ),
    ]
    .into_iter()
    {
        let (pattern, token) = parsing;
        let full_pattern = format!("^({})(?s)(\\s.*)$", pattern);

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

fn parse_macro_body<'a>(syntax: &'a str, tree: &mut Node, line_number: i32) -> Result<&'a str, &'a str> {
    let mut full_code = syntax;
    lazy_static! {
        static ref RE: Regex = Regex::new("^(\\s*)(?s)(.*)$").unwrap();
    }
    let token_kind = TokenKind::Whitespace;
    let result = parse_capture!(full_code, RE, token_kind, line_number, false);

    if let Some(result_parsed) = result {
        full_code = (result_parsed.1).1;
    } else {
        return Err("parsing error");
    }
    let mut open_character = &full_code[..1];

    let mut new_line_number = line_number;
    let mut begin_macro = 0;
    let mut end_macro = 0;
    let mut code = full_code;
    let mut macro_body = vec![];
    let mut chari = "";
    loop {
        chari = &code[..1];
        if chari == open_character {
            begin_macro += 1;
        }
        if chari == block_keyword::end_character(open_character) {
            end_macro += 1;
        }
        if chari == "\n" {
            new_line_number += 1;
        }
        code = &code[1..];
        macro_body.push(chari);

        if begin_macro == end_macro && begin_macro > 0 && end_macro > 0 {
            full_code = code;
            if &full_code[..1] == "\n" {
                macro_body.push(";");
            }
            tree.children.push(
                Node {
                    token: Token::new(TokenKind::MacroBody, new_line_number, false),
                    children: vec![],
                    data: Some(macro_body.join(""))
                }
            );
            break;
        }
    }

    Ok(full_code)
}