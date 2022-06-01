// This parser finds all instances of uniform variables (say iResolution) and inserts the
// "uni." root variable name (e.g. uni.iResolution)

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;

use nom::combinator::{eof, map};
use nom::error::VerboseError;
use nom::multi::{many0, many_till};

use nom::IResult;
use nom::Parser;

pub use crate::nom_helpers::{IResult2, Span};

pub type ParserResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

pub fn replace_uni(i: &str) -> ParserResult<String> {
    map(many_till(anychar, replace_tag), |(v, s)| {
        let mut v2 = v.iter().collect::<String>();
        v2.push_str(&s);
        v2
    })(i)
}

pub fn replace_tag(i: &str) -> ParserResult<String> {
    map(
        alt((
            tag("iResolution"),
            tag("iFrame"),
            tag("iTime"),
            tag("iTimeDelta"),
            tag("iSampleRate"),
            tag("iMouse"),
            tag("iChannelTime"),
            tag("iChannelResolution"),
            tag("iDate"),
        )),
        |x| {
            let mut u = "uni.".to_string();
            u.push_str(x);
            u
        },
    )(i)
}

pub fn replace_all_unis(i: &str) -> ParserResult<String> {
    map(many0(replace_uni), |x2| x2.join(""))(i)
}

pub fn uniform_vars_parser(i: &str) -> ParserResult<String> {
    map(
        replace_all_unis.and(many_till(anychar, eof)),
        |(mut replaced_unis, (rest, _))| {
            let rest_of_script: String = rest.iter().collect();
            replaced_unis.push_str(&rest_of_script);
            replaced_unis
        },
    )(i)
}
