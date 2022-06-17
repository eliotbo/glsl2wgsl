// use glsl2wgsl::parser::Parse;
use glsl2wgsl::nom_helpers::Span;
use glsl2wgsl::parser::Parse;
use glsl2wgsl::syntax;
// use glsl::*;
use glsl2wgsl::let2var::{let2var_parser, search_for_full_identifier};
use glsl2wgsl::replace_defines::definition_parser;
use glsl2wgsl::replace_main::replace_main_line;
use glsl2wgsl::replace_unis::uniform_vars_parser;
use glsl2wgsl::transpiler::wgsl::show_translation_unit;
// use glsl2wgsl::var_private_parser::add_private_to_global_vars;
use glsl2wgsl::parse_func_defines::func_definition_parser;
use glsl2wgsl::replace_inouts::{replace_inouts, search_and_replace_void};
use glsl2wgsl::replace_texel_fetch::*;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{alpha1, alphanumeric1, anychar, char, multispace0};

use nom::combinator::{eof, map, peek, recognize, success, verify};
use nom::error::VerboseError;
use nom::multi::{many0, many_till, separated_list0};
use nom::sequence::{delimited, pair, preceded};
use nom::Parser;

use std::fs;

const ADD_SPAN: &str = "
    assert_eq!(comment(\"// lolfoo\"), Ok((\"foo\", \" lol\")));
    assert_eq!(comment(\"// lol\\foo\"), Ok((\"\", \" lol\\foo\")));
    assert_eq!(
        comment(\"// lol   \\  foo\"),
        Ok((\"\", \" lol   \\  foo\"))
    );
}";

// TODO: fix the newline for statements
// THEN: big clean
use glsl2wgsl::parsers_span::*;

fn main() {
    // let mut replaced_quotes: String;

    let tests = fs::read_to_string("src/parse_tests.rs").expect("couldn't");

    // println!("HEREREREREE: {:?}", comment(Span::new("// lol")));

    // let (rest, (is_real, first_quote)) = find_one_quote(&ADD_SPAN).unwrap();
    // println!("{:?}", is_real);
    // println!("{:?}", rest);
    // println!("{:?}", first_quote);

    // let (rest, replaced_quotes) = find_quote_pair(&ADD_SPAN).unwrap();
    // let (rest, replaced_quotes) = replace_all_quote_pairs(&ADD_SPAN).unwrap();

    let (rest, replaced_quotes) = replace_all_quote_pairs(&tests).unwrap();

    // fs::write("./parse_span_texts.txt", &replaced_quotes).expect("Unable to write file");
}

fn replace_all_quote_pairs(i: &str) -> ParserResult<String> {
    map(
        many_till(
            find_quote_pair,
            // map(many_till(anychar, eof), |x| x.0.iter().collect::<String>()),
            eof,
        ),
        |(parsed, rest)| {
            //
            let mut ret = parsed.join("");
            ret += &rest;
            ret
            // "".to_string()
        },
    )(i)
}

fn find_quote_pair(i: &str) -> ParserResult<String> {
    let mut found_real_quote = false;
    let mut so_far: String = String::new();
    let mut rest = "";
    let mut is_eof_global = false;

    while !found_real_quote && !is_eof_global {
        let (new_rest, (is_real_quote, is_eof, parsed_text)) = find_one_quote(i)?;
        rest = new_rest;
        so_far += &parsed_text;
        is_eof_global = is_eof;
        found_real_quote = is_real_quote;
    }

    let mut within_quote_so_far: String = String::new();
    let mut within_quote_rest = "";

    let mut found_real_quote2 = false;
    while !found_real_quote2 && !is_eof_global {
        let (new_rest, (is_real_quote, is_eof, parsed_text)) = find_one_quote(rest)?;
        within_quote_rest = new_rest;
        within_quote_so_far += &parsed_text;

        is_eof_global = is_eof;
        found_real_quote2 = is_real_quote;
    }

    let mut ret = String::new();
    ret.push_str(&so_far);
    if !is_eof_global {
        ret += "Span::new(\"";
        ret += &within_quote_so_far;
        ret += "\")";
    }

    success(ret)(within_quote_rest)
}

// return type is (is_real_quote, is_eof, parsed_text)
fn find_one_quote(i: &str) -> ParserResult<(bool, bool, String)> {
    map(
        many_till(anychar, alt((tag("\""), eof))),
        |(before, quote_or_eof)| {
            let string_before = before.iter().collect::<String>();
            if quote_or_eof == "" {
                return (false, true, string_before);
            }
            if let Some(last_char) = string_before.chars().last() {
                if last_char == '\\' {
                    return (false, false, string_before + "\"");
                }
            }

            return (true, false, string_before);
        },
    )(i)
}

// fn replace_quotes_span(i: &str)  -> ParserResult<String> {

// }
