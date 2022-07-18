// replaces the #defines lines that have no arguments with
// a var<private> declaration

use crate::nom_helpers::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{anychar, space0, space1};
use nom::combinator::{map, success};
use nom::multi::{many0, many_till};
use nom::sequence::tuple;
use nom::Parser;

fn get_one_define(i: &str) -> ParserResult<String> {
    map(
        take_while1(identifier_num_pred).and(tuple((space0, tag("\n")))),
        |x: (&str, (&str, &str))| x.0.to_string(),
    )(i)
}

fn find_non_comment_define_tag(i: &str) -> ParserResult<String> {
    map(
        many_till(anychar, tuple((space0, tag("#define"), space1))).and(get_one_define),
        |((so_far, _), name)| {
            // let name = x.0 .0.to_string();
            // "let ".to_string() + &name + ": bool = true;\n"
            so_far.iter().collect::<String>() + "bool  " + &name + " = true;\n"
        },
    )(i)
}

fn find_commented_define_tag(i: &str) -> ParserResult<String> {
    map(
        // many_till(
        //     anychar,
        //     space0.and(tag("//")).and(space0).and(tag("#define")),
        // )
        many_till(
            anychar,
            tuple((space0, tag("//"), space0, tag("#define"), space1)),
        )
        .and(get_one_define),
        // .and(preceded(space1, get_one_define)),
        |((so_far, _), name)| {
            // let name = x.0 .0.to_string();
            so_far.iter().collect::<String>() + "bool " + &name + " = false;\n"
        },
    )(i)
}

pub fn defs_parser(i: &str) -> ParserResult<String> {
    let mut new_script = "".to_string();

    if let Ok((_rest, commented_replaced_defs)) = map(
        many0(find_commented_define_tag).and(rest_of_script),
        |(xs, ros)| xs.join("") + &ros,
    )(i)
    {
        new_script = commented_replaced_defs;
    }

    let new_script_clone = new_script.clone();

    if let Ok((_rest, non_commented_replaced_defs)) = map(
        many0(find_non_comment_define_tag).and(rest_of_script),
        |(xs, ros)| xs.join("") + &ros,
    )(&new_script_clone)
    {
        new_script = non_commented_replaced_defs;
    }

    success(new_script)("")
}

// use nom to parse the #ifdef #elseif #else and #endif keywords and turn
// them into if and else statements
fn replace_ifdefs(i: &str) -> ParserResult<String> {
    map(
        many_till(anychar, tuple((tag("#ifdef"), space1))).and(anychar_underscore),
        |((sofar, _), iden)| sofar.iter().collect::<String>() + "if (" + &iden + ") {",
    )(i)
}

fn replace_defelse_defend(i: &str) -> ParserResult<String> {
    map(
        many_till(anychar, alt((tag("#else"), tag("#endif")))),
        |(sofar, t)| {
            if t == "#else" {
                sofar.iter().collect::<String>() + "} else {"
            } else {
                sofar.iter().collect::<String>() + "}"
            }
        },
    )(i)
}

pub fn ifdefs_parser(i: &str) -> ParserResult<String> {
    let mut buf = "".to_string();

    if let Ok((_rest, replaced_ifdefs)) =
        map(many0(replace_ifdefs).and(rest_of_script), |(xs, ros)| {
            xs.join("") + &ros
        })(i)
    {
        buf = replaced_ifdefs;
    }

    let buf_clone = buf.clone();
    if let Ok((_rest, replaced_defelse_defend)) = map(
        many0(replace_defelse_defend).and(rest_of_script),
        |(xs, ros)| xs.join("") + &ros,
    )(&buf_clone)
    {
        buf = replaced_defelse_defend;
    }
    success(buf)("")
}
