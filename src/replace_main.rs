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
use nom::character::complete::{
    anychar, char, digit1, line_ending, multispace0, multispace1, space0, space1,
};
use nom::character::{is_hex_digit, is_oct_digit};
use nom::combinator::{cut, eof, map, not, opt, peek, recognize, success, value, verify};
use nom::error::{ErrorKind, ParseError as _, VerboseError, VerboseErrorKind};
use nom::multi::{count, fold_many0, many0, many1, many_till, separated_list0, separated_list1};
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use nom::Parser;

use nom::IResult;

pub use crate::nom_helpers::{many0__span, IResult2, Span};

pub type ParserResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

fn identifier_num_pred(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == '.'
}
fn identifier_type_pred(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == '<' || ch == '>'
}

fn func_pred(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == '(' || ch == ' ' || ch == ')' || ch == ','
}

pub fn anychar_underscore(i: &str) -> ParserResult<String> {
    map(take_while1(identifier_num_pred), |v: &str| v.to_string())(i)
}

pub fn anychar_type(i: &str) -> ParserResult<String> {
    map(take_while1(identifier_type_pred), |v: &str| v.to_string())(i)
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

// fn update([[builtin(global_invocation_id)]] invocation_id: vec3<u32>) {

// pub fn replace_main_vars(i: &str) -> ParserResult<String> {
//     map(count(anychar, 3), |s| s.iter().collect::<String>())(i)
// }

// fn parser(i: &str) -> ParserResult<Vec<String>> {
//     map(
//         separated_list0(tag("|"), many_till(anychar, tag(":"))),
//         |x| vec!["1".to_string(), "2".to_string()],
//     )(i)
// }

const UPDATE_INIT: &str = r"[[stage(compute), workgroup_size(8, 8, 1)]]
fn update([[builtin(global_invocation_id)]] invocation_id: vec3<u32>) {
    let R: vec2<f32> = uni.iResolution.xy;
    let y_inverted_location = vec2<i32>(i32(invocation_id.x), i32(R.y) - i32(invocation_id.y));
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    ";

pub fn replace_main(i: &str) -> ParserResult<()> {
    map(tag("fn mainImage("), |s: &str| return)(i)
}

pub fn parse_var_and_type(i: &str) -> ParserResult<String> {
    map(
        separated_pair(anychar_underscore, tag(": "), anychar_type),
        |s| s.0,
    )(i)
}

pub fn replace_main_vars(i: &str) -> ParserResult<Vec<String>> {
    map(
        terminated(
            separated_list1(tag(", "), parse_var_and_type),
            tag(") -> () {"),
        ),
        |s| s,
    )(i)
}

pub fn replace_main_line(i: &str) -> ParserResult<String> {
    map(
        replace_main
            .and(replace_main_vars)
            .and(many_till(anychar, eof)),
        |(s, rest)| {
            let mut s2 = UPDATE_INIT.to_string();
            let mut vars = s.1.iter();

            if let Some(u) = vars.next() {
                s2.push_str(&format!("\n\tvar {}: vec4<f32>;", u));
            }

            if let Some(pos) = vars.next() {
                s2.push_str(&format!(
                    "\n\tvar {} = vec2<f32>(f32(location.x), f32(location.y) );\n",
                    pos
                ));
            }

            s2.push_str(&rest.0.iter().collect::<String>());
            s2
        },
    )(i)
}

// fn function_call_args(i: &str) -> ParserResult<String> {
//     map(
//         preceded(
//             terminated(terminated(blank_space, char('(')), blank_space),
//             alt((
//                 map(
//                     terminated(blank_space, terminated(blank_space, char(')'))),
//                     |_| vec![],
//                 ),
//                 terminated(
//                     separated_list0(
//                         terminated(char(','), blank_space),
//                         cut(terminated(anychar_func, blank_space)),
//                     ),
//                     cut(char(')')),
//                 ),
//             )),
//         ),
//         |x| {
//             //
//             "".to_string()
//         },
//     )(i)
// }

fn function_call_args(i: &str) -> ParserResult<String> {
    map(
        char('(')
            .and(separated_list0(
                terminated(char(','), multispace0),
                anychar_underscore,
            ))
            .and(char(')')), // many0(terminated(anychar, terminated(tag(","), blank_space))),
        |x| {
            //
            println!("function_call_args: {:?}", x);
            "".to_string()
        },
    )(i)
}

// // pub fn parse_expr(i: &str) -> ParserResult<String> {}

// pub fn parse_func_define(i: &str) -> ParserResult<String> {
//     map(
//         // function_call_args.and(space0).and(function_call_args),
//         function_call_args.and(parse_expr),
//         // .and(take_while1(identifier_num_pred)),
//         |x| {
//             // let mut name = "let ".to_string();
//             // name.push_str(x.0 .0);
//             // name.push_str(" = ");
//             // name.push_str(x.1);
//             // name.push(';');
//             // name
//             // println!("{:?}", x);
//             "".to_string()
//         },
//     )(i)
// }

// pub fn replace_tag(i: &str) -> ParserResult<String> {
//     map(
//         preceded(tag("#define").and(space1), parse_func_define),
//         |x| x,
//     )(i)
// }

pub fn replace_tag(i: &str) -> ParserResult<String> {
    map(
        preceded(tag("#define").and(space1), anychar_underscore).and(function_call_args),
        |x| {
            //
            println!("{:?}", x);
            "".to_string()
        },
    )(i)
}

pub fn replace_define_func(i: &str) -> ParserResult<String> {
    map(many_till(anychar, replace_tag), |(v, s)| {
        let mut v2 = v.iter().collect::<String>();

        v2.push_str(&s);
        v2
    })(i)
}

pub fn replace_all_define_funcs(i: &str) -> ParserResult<String> {
    map(many0(replace_define_func), |x2| x2.join(""))(i)
}

pub fn func_definition_parser(i: &str) -> ParserResult<String> {
    map(
        replace_all_define_funcs.and(many_till(anychar, eof)),
        |(mut replaced_definitions, (rest, _))| {
            let rest_of_script: String = rest.iter().collect();
            replaced_definitions.push_str(&rest_of_script);
            replaced_definitions
        },
    )(i)
}
