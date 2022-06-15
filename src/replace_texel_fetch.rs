// This parser finds and replaces all instances of the GLSL texelFetch and texture functions
// by the WGLSL textureLoad function.

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{alpha1, alphanumeric1, anychar, char, multispace0};

use nom::combinator::{eof, map, peek, recognize, verify};
use nom::error::VerboseError;
use nom::multi::{many0, many_till, separated_list0};
use nom::sequence::{delimited, pair, preceded};
use nom::Parser;

use nom::IResult;

pub use crate::nom_helpers::*;

pub type ParserResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

fn identifier_hashtag(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == '#'
}

fn identifier_num_pred(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == '.'
}

fn func_pred(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == '(' || ch == ' ' || ch == ')' || ch == ','
}

pub fn anychar_underscore(i: &str) -> ParserResult<String> {
    map(take_while1(identifier_num_pred), |v: &str| v.to_string())(i)
}

pub fn anychar_underscore_hashtag(i: &str) -> ParserResult<String> {
    map(take_while1(identifier_hashtag), |v: &str| v.to_string())(i)
}

pub fn anychar_func(i: &str) -> ParserResult<String> {
    map(take_while1(func_pred), |v: &str| v.to_string())(i)
}

pub fn identifier(input: &str) -> ParserResult<&str> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        ))
        // make sure the next character is not part of the identifier
        .and(peek(verify(anychar, |x| !x.is_alphanumeric()))),
        |x: (&str, char)| {
            let ret = x.0;
            ret
        },
    )(input)
}

pub fn rest_of_script(i: &str) -> ParserResult<String> {
    map(many_till(anychar, eof), |x| x.0.iter().collect())(i)
}

// all characters until either a comma or a parenthesis
pub fn till_next_paren_or_comma(i: &str) -> ParserResult<(String, String)> {
    map(
        many_till(anychar, alt((tag("("), tag(")"), tag(",")))),
        |(so_far, brack): (Vec<char>, &str)| {
            //
            let text = so_far.iter().collect::<String>();
            return (text, brack.to_string());
        },
    )(i)
}

// parse one argument of a function call
pub fn argument1(i: &str) -> ParserResult<String> {
    let mut parsed_text: String = "".to_string();
    let mut scope = 0;
    let mut rest = i;
    loop {
        let (rest1, (text_so_far, paren_or_comma)): (&str, (String, String)) =
            till_next_paren_or_comma(rest)?;
        rest = rest1;
        parsed_text += &text_so_far;

        match paren_or_comma.as_str() {
            "(" => scope += 1,
            ")" => {
                scope -= 1;

                // end of function call
                if scope == -1 {
                    break;
                }
            }
            _ => {
                // case of a comma
                // end of argument
                if scope == 0 {
                    break;
                }
            }
        }

        parsed_text += &paren_or_comma;
    }

    Ok((rest, parsed_text))
}

pub fn function_call_args_anychar(i: &str) -> ParserResult<Vec<String>> {
    map(
        preceded(
            tag("("),
            many0(delimited(multispace0, argument1, multispace0)),
        ),
        |x: Vec<String>| x,
    )(i)
}

pub fn function_call_args(i: &str) -> ParserResult<Vec<String>> {
    map(
        delimited(
            tag("("),
            separated_list0(
                delimited(multispace0, char(','), multispace0),
                anychar_underscore,
            ),
            tag(")"),
        ),
        |x| x,
    )(i)
}

// search (v, where v is an identifier) and replace by (num, which can be anychar)
pub fn search_and_replace_identifier(i: &str, v: String, num: String) -> ParserResult<String> {
    map(
        many_till(
            many_till(anychar, alt((verify(identifier, |x| x == v), eof))),
            eof,
        ),
        |x| {
            // makes sure that the identifier does not have any other alphanum characters
            // before and after it
            let mut ret = "".to_string();
            for (v_chars, name) in x.0.iter() {
                ret.push_str(&v_chars.iter().collect::<String>());
                if let Some(c) = ret.chars().last() {
                    if name == &v && !c.is_alphanumeric() {
                        ret.push_str(&num);
                    } else {
                        ret.push_str(&name);
                    }
                } else {
                    ret.push_str(&name);
                }
                // ret.push_str(&num);
            }
            ret
        },
    )(i)
}

pub fn till_next_brace(i: &str) -> ParserResult<(String, String)> {
    map(
        many_till(anychar, alt((tag("{"), tag("}")))),
        |(so_far, brack): (Vec<char>, &str)| {
            //

            let text = so_far.iter().collect::<String>();
            return (text, brack.to_string());
        },
    )(i)
}

pub fn check_one_texel_fetch(i: &str) -> ParserResult<String> {
    let (rest, arguments) = map(
        many_till(
            anychar,
            preceded(tag("texelFetch"), function_call_args_anychar),
        ),
        |(before_texel_fetch, args)| {
            let mut ret = before_texel_fetch.iter().collect::<String>();

            // ret = ret + "textureLoad(" + args[0].as_str() + ", ";
            ret = ret + "textureLoad(" + "BUFFER_" + args[0].as_str() + ", ";
            ret = ret + "vec2<i32>(" + args[1].as_str() + "))";

            ret
        },
    )(i)?;

    return Ok((rest, arguments));
}

pub fn check_one_texture(i: &str) -> ParserResult<String> {
    let (rest, arguments) = map(
        many_till(
            anychar,
            preceded(tag("texture"), function_call_args_anychar),
        ),
        |(before_texel_fetch, args)| {
            let mut ret = before_texel_fetch.iter().collect::<String>();

            // ret = ret + "textureLoad(" + args[0].as_str() + ", ";
            ret = ret + "textureLoad(" + "BUFFER_" + args[0].as_str() + ", ";
            ret = ret
                + "vec2<i32>("
                + args[1].as_str()
                + "  /* 0 to 1 range -> CONVERT TO I32 */  ))";

            ret
        },
    )(i)?;

    return Ok((rest, arguments));
}

pub fn replace_all_texture_and_texel_fetch(i: &str) -> ParserResult<String> {
    let (_, replaced_texel_fetch) = map(
        many0(check_one_texel_fetch).and(many_till(anychar, eof)),
        |(s, (t, _q))| {
            let so_far = s.join("");
            let rest = t.iter().collect::<String>();

            so_far + &rest
        },
    )(i)?;

    let replaced_texel_fetch_and_texture = map(
        many0(check_one_texture).and(many_till(anychar, eof)),
        |(s, (t, _q))| {
            let so_far = s.join("");
            let rest = t.iter().collect::<String>();

            so_far + &rest
        },
    )(replaced_texel_fetch.as_str());

    if let Ok((_rest, r)) = replaced_texel_fetch_and_texture {
        return Ok(("", r));
    } else {
        return Ok(("", i.to_string()));
    }
}
