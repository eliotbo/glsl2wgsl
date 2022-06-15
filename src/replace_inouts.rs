// This parser finds function arguments with the "inout" storage qualifier and replaces them
// with pointers throughout the function body.

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{alpha1, alphanumeric1, anychar, char, multispace0, one_of};

use nom::combinator::{eof, map, peek, recognize, verify};
use nom::error::VerboseError;
use nom::multi::{many0, many_till, separated_list0};
use nom::sequence::{delimited, pair, preceded};
use nom::Parser;

use nom::IResult;

// pub use crate::nom_helpers::{many0__span, IResult2, Span};
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

pub fn argument1(i: &str) -> ParserResult<String> {
    map(many_till(anychar, peek(one_of(",)"))), |x| {
        x.0.iter().collect::<String>()
        // &s
    })(i)
}

pub fn function_call_args_anychar(i: &str) -> ParserResult<Vec<String>> {
    map(
        delimited(
            tag("("),
            separated_list0(
                delimited(multispace0, char(','), multispace0),
                alt((argument1, map(multispace0, |x: &str| x.to_string()))),
            ),
            tag(")"),
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

pub fn search_and_replace_void(i: &str) -> ParserResult<String> {
    map(
        many_till(many_till(anychar, alt((tag("-> ()"), eof))), eof),
        |x| {
            //
            let mut ret = "".to_string();
            for (v_chars, _) in x.0.iter() {
                ret.push_str(&v_chars.iter().collect::<String>());
                // if name == &v {
                //     ret.push_str(&num);
                // }
            }
            ret
        },
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

pub fn get_function_body(i: &str, mut scope: u32) -> ParserResult<String> {
    let mut parsed_text: String = "".to_string();
    let mut rest = i;
    loop {
        let (rest1, (text_so_far, brace)): (&str, (String, String)) = till_next_brace(rest)?;
        rest = rest1;
        parsed_text += &text_so_far;
        parsed_text += &brace;

        if brace == "{" {
            scope += 1;
        } else {
            scope -= 1;
        }

        if scope == 0 {
            break;
        }
    }

    return Ok((rest, (parsed_text)));
}

pub fn check_inout_arg(i: &str) -> ParserResult<String> {
    map(preceded(tag("inout "), anychar_underscore), |x| {
        x.to_string()
    })(i)
}

// add type ptr<function, particle>
pub fn add_ptr_to_arg(i: &str) -> ParserResult<String> {
    map(
        many_till(anychar, tag(": ")).and(many_till(anychar, eof)),
        |((name, _), (vc, _))| {
            let arg_name = name.iter().collect::<String>();
            let type_name = vc.iter().collect::<String>();
            return arg_name + ": ptr<function, " + &type_name + ">";
        },
    )(i)
}

pub fn check_one_func(i: &str) -> ParserResult<String> {
    let (_, arguments) = map(
        peek(many_till(
            anychar,
            preceded(tag("fn "), identifier).and(function_call_args_anychar),
        )),
        |(_s, (_iden, args))| args,
    )(i)?;

    // delete "inout" keywords and add type ptr<function, particle>,
    let (rest, parsed_func_def) = map(
        many_till(
            anychar,
            preceded(tag("fn "), identifier).and(function_call_args_anychar),
        ),
        |(before_func, (func_name, args))| {
            //
            let args_joined = args
                .iter()
                .map(|maybe_inout| {
                    //
                    if let Some(no_inout) = maybe_inout.strip_prefix("inout ") {
                        let stripped_inout = no_inout.to_string();

                        if let Ok((_, added_ptr_type)) = add_ptr_to_arg(&stripped_inout) {
                            added_ptr_type
                        } else {
                            "ERROR: could not parser TYPE of inout arg".to_string()
                        }
                    } else {
                        maybe_inout.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join(", ");
            before_func.iter().collect::<String>() + &func_name + "(" + &args_joined + ")"
        },
    )(i)?;

    let (rest, mut body) = get_function_body(rest, 0)?;

    for arg in arguments.iter() {
        if let Ok((_, inout_arg_name)) = check_inout_arg(arg) {
            let ptr = "(*".to_string() + &inout_arg_name + ")";
            if let Ok((_, body2)) = search_and_replace_identifier(&body, inout_arg_name, ptr) {
                body = body2;
            }
        }
    }

    return Ok((rest, parsed_func_def.to_string() + &body));
}

pub fn replace_inouts(i: &str) -> ParserResult<String> {
    map(
        many0(check_one_func).and(many_till(anychar, eof)),
        |(vec_parsed, rest)| vec_parsed.join("") + &rest.0.iter().collect::<String>(),
    )(i)
}
