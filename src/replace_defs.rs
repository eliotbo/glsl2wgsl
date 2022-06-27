// replaces the #defines lines that have no arguments with
// a var<private> declaration

use crate::nom_helpers::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{anychar, none_of, space0, space1};
use nom::combinator::{eof, map, recognize, success, verify};
use nom::multi::{count, many0, many_till};
use nom::sequence::{pair, preceded, tuple};
use nom::Parser;

#[derive(Debug)]
pub struct HashDefine {
    pub name: String,
    pub replace_by: String,
}

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
