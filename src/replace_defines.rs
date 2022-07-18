// replaces the #defines lines that have no arguments with
// a var<private> declaration

use crate::nom_helpers::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{anychar, none_of, space1};
use nom::combinator::{eof, map, success, verify};
use nom::multi::{count, many0, many_till};
use nom::sequence::{pair, preceded};
use nom::Parser;

#[derive(Debug)]
pub struct HashDefine {
    pub name: String,
    pub replace_by: String,
}

pub fn get_one_define(i: &str) -> ParserResult<HashDefine> {
    map(
        take_while1(identifier_num_pred)
            .and(space1)
            .and(many_till(anychar, tag("\n"))),
        |x: ((&str, &str), (Vec<char>, &str))| HashDefine {
            name: x.0 .0.to_string(),
            replace_by: x.1 .0.iter().collect::<String>(),
        },
    )(i)
}

pub fn find_define_tag(i: &str) -> ParserResult<HashDefine> {
    preceded(
        many_till(anychar, tag("#define")).and(space1),
        get_one_define,
    )(i)
}

pub fn find_and_replace_single_define(
    i: &str,
    name: String,
    replace_by: String,
) -> ParserResult<String> {
    map(
        many0(many_till(
            anychar,
            pair(
                count(none_of(ALPHANUM_UNDER), 1),
                verify(anychar_underscore_dot, |x: &str| x.to_string() == name),
            ),
        ))
        .and(rest_of_script),
        |(lines, rest)| {
            let mut all_script = "".to_string();
            for (so_far_chars, (single_char, _name)) in lines.iter() {
                let mut so_far = so_far_chars.iter().collect::<String>();
                so_far += &single_char.iter().collect::<String>();

                so_far.push_str(&replace_by);
                all_script.push_str(&so_far);
            }
            all_script.push_str(&rest);
            all_script
        },
    )(i)
}

pub fn erase_one_define(i: &str) -> ParserResult<String> {
    map(
        many_till(
            anychar,
            preceded(tag("#define "), many_till(anychar, tag("\n"))),
        ),
        |x| x.0.iter().collect(),
    )(i)
}

pub fn erase_all_defines(i: &str) -> ParserResult<String> {
    map(
        many_till(alt((erase_one_define, rest_of_script)), eof),
        |x| x.0.join(""),
    )(i)
}

pub fn definition_parser(i: &str) -> ParserResult<String> {
    let (_rest, hash_defines_info) = many0(find_define_tag)(i)?;

    let (_, no_defines) = erase_all_defines(i)?;

    let mut new_script = no_defines;

    for hash_define in hash_defines_info.iter() {
        if let Ok((_, new_script2)) = find_and_replace_single_define(
            &new_script,
            hash_define.name.clone(),
            hash_define.replace_by.clone(),
        ) {
            new_script = new_script2;
        }
    }

    success(new_script)("")
}
