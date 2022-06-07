// This parser finds all instances of uniform variables (say iResolution) and inserts the
// "uni." root variable name (e.g. uni.iResolution)

// use nom::branch::alt;
// use nom::bytes::complete::tag;
// use nom::character::complete::anychar;

// use nom::combinator::{eof, map};
// use nom::error::VerboseError;
// use nom::multi::{many0, many_till};

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while1};
use nom::character::complete::{anychar, char, digit1, line_ending, multispace1, space0, space1};
use nom::character::{is_hex_digit, is_oct_digit};
use nom::combinator::{cut, eof, map, not, opt, peek, recognize, success, value, verify};
use nom::error::{ErrorKind, ParseError as _, VerboseError, VerboseErrorKind};
use nom::multi::{fold_many0, many0, many1, many_till, separated_list0};
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use nom::Parser;

use nom::IResult;

pub use crate::nom_helpers::{IResult2, Span};

pub type ParserResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

pub fn replace_define(i: &str) -> ParserResult<String> {
    map(many_till(anychar, replace_tag), |(v, s)| {
        let mut v2 = v.iter().collect::<String>();
        v2.push_str(&s);
        v2
    })(i)
}

fn identifier_num_pred(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == '.'
}

pub fn anychar_underscore(i: &str) -> ParserResult<String> {
    map(take_while1(identifier_num_pred), |v: &str| v.to_string())(i)
}

pub fn parse_define(i: &str) -> ParserResult<String> {
    map(
        take_while1(identifier_num_pred)
            .and(space1)
            .and(take_while1(identifier_num_pred)),
        |x| {
            let mut name = "var<private> ".to_string();
            name.push_str(x.0 .0);
            name.push_str(" = ");
            name.push_str(x.1);
            name.push(';');
            name
        },
    )(i)
}

pub fn replace_tag(i: &str) -> ParserResult<String> {
    map(preceded(tag("#define").and(space1), parse_define), |x| x)(i)
}

pub fn replace_all_defines(i: &str) -> ParserResult<String> {
    map(many0(replace_define), |x2| x2.join(""))(i)
}

pub fn definition_parser(i: &str) -> ParserResult<String> {
    map(
        replace_all_defines.and(many_till(anychar, eof)),
        |(mut replaced_definitions, (rest, _))| {
            let rest_of_script: String = rest.iter().collect();
            replaced_definitions.push_str(&rest_of_script);
            replaced_definitions
        },
    )(i)
}
