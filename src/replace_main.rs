// This parser replaces the main function line, typically
// "fn mainImage( U: vec4<f32>,  pos: vec2<f32>) -> () {",
// by the UPDATE_INIT code block below

use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::anychar;
use nom::combinator::{eof, map, opt};
use nom::multi::{many_till, separated_list1};
use nom::sequence::{separated_pair, terminated};
use nom::Parser;

use crate::nom_helpers::*;

fn identifier_type_pred(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == '<' || ch == '>'
}

pub fn anychar_type(i: &str) -> ParserResult<String> {
    map(take_while1(identifier_type_pred), |v: &str| v.to_string())(i)
}

const UPDATE_INIT: &str = "[[stage(compute), workgroup_size(8, 8, 1)]]
fn update([[builtin(global_invocation_id)]] invocation_id: vec3<u32>) {
    let R: vec2<f32> = uni.iResolution.xy;
    let y_inverted_location = vec2<i32>(i32(invocation_id.x), i32(R.y) - i32(invocation_id.y));
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    ";

pub fn replace_main(i: &str) -> ParserResult<String> {
    map(many_till(anychar, tag("fn mainImage(")), |s| {
        s.0.clone().iter().collect::<String>()
    })(i)
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
        opt(replace_main
            .and(replace_main_vars)
            .and(many_till(anychar, eof))),
        |maybe| {
            if let Some(((before_main, vars), rest)) = maybe {
                let mut s2 = before_main;
                s2.push_str(&UPDATE_INIT.to_string());
                let mut vars_iter = vars.iter();

                if let Some(u) = vars_iter.next() {
                    s2.push_str(&format!("\n\tvar {}: vec4<f32>;", u));
                }

                if let Some(pos) = vars_iter.next() {
                    s2.push_str(&format!(
                        "\n\tvar {} = vec2<f32>(f32(location.x), f32(location.y) );\n",
                        pos
                    ));
                }

                s2.push_str(&rest.0.iter().collect::<String>());
                return s2;
            } else {
                return i.to_string();
            }
        },
    )(i)
}
