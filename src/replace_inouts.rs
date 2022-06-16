// This parser finds function arguments with the "inout" storage qualifier and replaces them
// with pointers throughout the function body.

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, multispace0, one_of};
use nom::combinator::{eof, map, peek};
use nom::multi::{many0, many_till, separated_list0};
use nom::sequence::{delimited, preceded};
use nom::Parser;

pub use crate::nom_helpers::*;

pub fn argument1_simple(i: &str) -> ParserResult<String> {
    map(many_till(anychar, peek(one_of(",)"))), |x| {
        x.0.iter().collect::<String>()
        // &s
    })(i)
}

pub fn function_call_args_anychar2(i: &str) -> ParserResult<Vec<String>> {
    map(
        delimited(
            tag("("),
            separated_list0(
                delimited(multispace0, char(','), multispace0),
                alt((argument1_simple, map(multispace0, |x: &str| x.to_string()))),
            ),
            tag(")"),
        ),
        |x: Vec<String>| x,
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
            }
            ret
        },
    )(i)
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
            preceded(tag("fn "), identifier).and(function_call_args_anychar2),
        )),
        |(_s, (_iden, args))| args,
    )(i)?;

    // delete "inout" keywords and add type ptr<function, particle>,
    let (rest, parsed_func_def) = map(
        many_till(
            anychar,
            preceded(tag("fn "), identifier).and(function_call_args_anychar2),
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
