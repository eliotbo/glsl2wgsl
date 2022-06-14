// This parser finds all instances of uniform variables (say iResolution) and inserts the
// "uni." root variable name (e.g. uni.iResolution)

// use nom::branch::alt;
// use nom::bytes::complete::tag;
// use nom::character::complete::anychar;

// use nom::combinator::{eof, map};
// use nom::error::VerboseError;
// use nom::multi::{many0, many_till};

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until, take_while1};
use nom::character::complete::{
    alpha1, alphanumeric1, anychar, char, digit1, line_ending, multispace0, multispace1, one_of,
    space0, space1,
};
use nom::character::{is_hex_digit, is_oct_digit};
use nom::combinator::{cut, eof, map, not, opt, peek, recognize, success, value, verify};
use nom::error::{ErrorKind, ParseError as _, VerboseError, VerboseErrorKind};
use nom::multi::{count, fold_many0, many0, many0_count, many1, many_till, separated_list0};
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use nom::Err::*;
use nom::Parser;

use nom::IResult;

// pub use crate::nom_helpers::{many0__span, IResult2, Span};
pub use crate::nom_helpers::*;

pub type ParserResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

const ALPHANUM_UNDER: &str = "abcdfghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789";

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

pub fn blank_space(i: &str) -> ParserResult<String> {
    map(recognize(many0(alt((multispace0, tag("\\\n"))))), |x| {
        "".to_string()
    })(i)
}

pub fn blank_space2(i: &str) -> ParserResult<String> {
    map(many0(alt((multispace0, tag("\t")))), |x| "".to_string())(i)
}

pub fn erase_one_define(i: &str) -> ParserResult<String> {
    map(
        many_till(
            anychar,
            preceded(
                tag("#define ")
                    .and(anychar_underscore)
                    .and(function_call_args),
                many_till(anychar, tag("\n")),
            ),
        ),
        // .and(rest_of_script),
        // |x| x.0 .0.iter().collect::<String>() + &x.1,
        |x| x.0.iter().collect(),
    )(i)
}

// pub fn identifier(input: &str) -> IResult<&str, &str> {
//   recognize(
//     pair(
//       alt((alpha1, tag("_"))),
//       many0_count(alt((alphanumeric1, tag("_"))))
//     )
//   )(input)
// }

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

pub fn erase_all_func_defines(i: &str) -> ParserResult<String> {
    map(
        many_till(alt((erase_one_define, rest_of_script)), eof),
        |x| x.0.join(""),
    )(i)
}

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

pub fn argument1(i: &str) -> ParserResult<String> {
    let mut parsed_text: String = "".to_string();
    let mut scope = 1;
    let mut rest = i;
    loop {
        let (rest1, (text_so_far, paren_or_comma)): (&str, (String, String)) =
            till_next_paren_or_comma(rest)?;
        rest = rest1;
        parsed_text += &(text_so_far + &paren_or_comma);

        match paren_or_comma.as_str() {
            "(" => scope += 1,

            ")" => scope -= 1,

            _ => {} // case of a comma
        }

        if scope == 0 {
            break;
        }
    }

    return Ok((rest, parsed_text));
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
        // |x: Vec<&str>| x.iter().map(|y| y.to_string()).collect::<Vec<String>>(),
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
pub fn search_and_replace(i: &str, v: String, num: String) -> ParserResult<String> {
    map(
        many_till(
            many_till(
                anychar,
                alt((
                    verify(anychar_underscore_hashtag, |x: &str| x.to_string() == v),
                    map(eof, |x: &str| x.to_string()),
                )),
            ),
            eof,
        ),
        |x| {
            //
            let mut ret = "".to_string();
            for (v_chars, name) in x.0.iter() {
                ret.push_str(&v_chars.iter().collect::<String>());
                if name == &v {
                    ret.push_str(&num);
                }
            }
            ret
        },
    )(i)
}

// search (v, where v is an identifier) and replace by (num, which can be anychar)
pub fn search_and_replace_identifier(i: &str, v: String, num: String) -> ParserResult<String> {
    map(
        many_till(
            many_till(
                anychar,
                // alt((tag(&*v), eof)),
                // verify(identifier, |(x, id)| x.to_string() == v),
                // alt(identifier_check(v.to_string())),
                alt((verify(identifier, |x| x == v), eof)),
            ),
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

pub fn replace_tag(i: &str) -> ParserResult<(String, Vec<String>)> {
    map(
        preceded(tag("#define").and(space1), anychar_underscore).and(function_call_args),
        |x| x,
    )(i)
}

#[derive(Debug, Clone)]
pub struct DefineFunc {
    pub name: String,
    pub args: Vec<String>,
    pub replace_by: String,
}

// pub fn get_name_and_args(i: &str) -> ParserResult<(String, Vec<String>)> {
pub fn get_name_and_args(i: &str) -> ParserResult<(String, Vec<String>)> {
    map(many_till(anychar, replace_tag), |(v, (name, args))| {
        (name, args)
    })(i)
}

// TODO: right now, the end of a defined function is detected with a newline char.
// Perhaps make more robust... not sure how
pub fn get_assignment(i: &str) -> ParserResult<String> {
    map(preceded(multispace0, many_till(anychar, tag("\n"))), |x| {
        x.0.iter().collect()
    })(i)
}

pub fn construct_assignment_vars(i: &str) -> ParserResult<DefineFunc> {
    let (rest, (name, args)) = get_name_and_args(i)?;
    let (rest, mut assignment) = get_assignment(rest)?;

    for (num, arg) in args.iter().enumerate() {
        let num_str = "#arg_".to_string() + &num.to_string();
        // println!("arg: {}", arg);
        if let Ok((_, assignment2)) =
            search_and_replace_identifier(&assignment, arg.to_string(), num_str)
        {
            // println!("assignment2assignment2assignment2",);
            assignment = assignment2;
        }
    }

    let define_func = DefineFunc {
        name,
        args,
        replace_by: assignment,
    };

    return Ok((rest, define_func));
}

pub fn get_all_define_funcs(i: &str) -> ParserResult<Vec<DefineFunc>> {
    map(many0(construct_assignment_vars), |x2| {
        // let mut assignments = x2.join("\n");
        // assignments.push('\n');
        // assignments
        x2
    })(i)
}

pub fn detect_identifier_as_arg(i: &str, name: String) -> ParserResult<String> {
    map(
        verify(anychar_underscore, |x: &str| x.to_string() == name),
        |x| {
            //
            x
        },
    )(i)
}

pub fn find_and_replace_single_define_func(i: &str, def: DefineFunc) -> ParserResult<String> {
    map(
        many0(
            many_till(
                anychar,
                verify(anychar_underscore, |x: &str| x.to_string() == def.name),
            )
            .and(function_call_args_anychar),
        )
        .and(rest_of_script),
        // many0(many_till(anychar, (tag(&*def.name))).and(function_call_args)).and(rest_of_script),
        |(lines, rest)| {
            //
            // println!(  "success : {:?}", rest );

            let mut all_script = "".to_string();
            for ((so_far_chars, _), args) in lines.iter() {
                // println!("arggs : {:?}", args);
                let mut so_far = so_far_chars.iter().collect::<String>();
                let mut replaced_expression = def.replace_by.to_string();

                for (n, arg) in args.iter().enumerate() {
                    let num_str = "#arg_".to_string() + &n.to_string();
                    if let Ok((_, assignment)) =
                        search_and_replace(&replaced_expression, num_str.clone(), arg.to_string())
                    {
                        // println!("OKAY OKAY OKAY: {}, {}", arg, &num_str);
                        // println!("replaced_expression: {}, ", replaced_expression);
                        replaced_expression = assignment;
                    }
                }

                so_far.push_str(&replaced_expression);
                all_script.push_str(&so_far);
            }
            all_script.push_str(&rest);
            all_script
        },
    )(i)
}

pub fn find_and_replace_define_funcs(i: &str, defs: Vec<DefineFunc>) -> ParserResult<String> {
    let mut full_script = i.to_string();
    for def in defs.iter() {
        if let Ok((_, fs)) = find_and_replace_single_define_func(&full_script, def.clone()) {
            full_script = fs;
        }
    }
    // println!("full_script : {:?}", full_script);
    // println!("defs : {:?}", defs);
    success(full_script)("")

    // map(many0(construct_assignment_vars), |x2| {
    //     //
    //     "".to_string()
    // })(i)
}

// preceded( delimited(tag("#define "),  anychar_underscore,
//     delimited(tag("("), many0(anychar), tag(")"))
// ) )),

// TODO:
// replace definition instances using the argument order

pub fn func_definition_parser(i: &str) -> ParserResult<String> {
    let (rest, define_funcs) = get_all_define_funcs(i)?;
    println!("def : {:?}", define_funcs);
    let (_, no_defines) = erase_all_func_defines(i)?;
    // println!("define_funcs: {:?}", define_funcs);
    // println!("no_defines: {:?}", no_defines);

    if let Ok((rest, so_far)) = find_and_replace_define_funcs(&no_defines, define_funcs) {
        return success(so_far)("");
    }

    success("".to_string())("")
}

// pub fn func_definition_parser(i: &str) -> ParserResult<String> {
//     map(
//         get_all_define_funcs.and(many_till(anychar, eof)),
//         |(mut replaced_definitions, (rest, _))| {
//             let rest_of_script: String = rest.iter().collect();
//             replaced_definitions.push_str(&rest_of_script);
//             replaced_definitions
//         },
//     )(i)
// }
