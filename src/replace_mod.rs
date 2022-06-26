// This parser replaces the main function line, typically
// "fn mainImage( U: vec4<f32>,  pos: vec2<f32>) -> () {",
// by the UPDATE_INIT code block below

use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::anychar;
use nom::combinator::{eof, map, opt};
use nom::multi::{many0, many_till, separated_list1};
use nom::sequence::{separated_pair, terminated};
use nom::Parser;

use crate::nom_helpers::*;

pub fn replace_one_mod(i: &str) -> ParserResult<String> {
    map(
        many_till(anychar, tag("mod")).and(function_call_args_anychar),
        |(s, args)| {
            let so_far = s.0.clone().iter().collect::<String>();
            so_far + "((" + &args[0].clone() + ") % (" + &args[1].clone() + "))"
        },
    )(i)
}

pub fn replace_all_mods(i: &str) -> ParserResult<String> {
    map(
        many0(replace_one_mod).and(many_till(anychar, eof)),
        |(mods, (rest, _))| {
            let a = mods.join("");
            a + &rest.iter().collect::<String>()
        },
    )(i)
}
