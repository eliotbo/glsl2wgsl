//! Notes:
//!
//! The func identifier for a "#define range(..) (replacement)" needs to appear with
//! a non-replaced non-identifier character right before it. For example,
//! this will work:  range(i, -2, 2) range(j, -2, 2)
//! but this won't:  range(i, -2, 2)range(j, -2, 2)
//! because the last character in the first range(..) -- namely ')' -- will disappear
//! after being replaced.

#![allow(unsafe_code)]
use parser::Parse;
use wasm_bindgen::prelude::*;

pub mod let2var;

pub mod parser;
#[cfg(test)]
// mod parse_tests;
mod wgsl_convert_test;
// pub mod parsers;
pub mod parsers_span;

pub mod syntax;
pub mod transpiler;

pub mod insert_new_arg_vars;
pub mod nom_helpers;
pub mod parse_func_defines;
pub mod replace_defines;
pub mod replace_inouts;
pub mod replace_main;
pub mod replace_texel_fetch;
pub mod replace_unis;

use insert_new_arg_vars::add_var_to_reassigned_args;
use parse_func_defines::*;
use replace_defines::*;
use replace_inouts::{replace_inouts, search_and_replace_void};
use replace_texel_fetch::replace_all_texture_and_texel_fetch;
use replace_unis::*;

use let2var::let2var_parser;
use nom::combinator::success;
use nom::error::{VerboseError, VerboseErrorKind};
use nom_helpers::ParserResult;
use parsers_span::Span;
use replace_main::replace_main_line;

#[wasm_bindgen]
extern "C" {
    pub fn prompt(s: &str, o: &str) -> String;
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(_v: &str) {}

pub fn preprocessing(i: &str) -> ParserResult<String> {
    let (_rest, replaced_defines_func) = func_definition_parser(&i)?;
    if let Ok((_rest, replaced_defines)) = definition_parser(replaced_defines_func.as_str()) {
        return success(replaced_defines)("");
    } else {
        let vek = VerboseErrorKind::Context("Could not properly parse the #define functions");
        let _ve = VerboseError {
            errors: vec![(_rest, vek)],
        };
        Err(nom::Err::Failure(_ve))
    }
}

pub fn postprocessing(i: &str) -> String {
    // the following parsers cannot fail, so we can use unwrap freely
    let mut buf: String;

    // println!("{:?}", i);

    if let Ok((_, lets)) = let2var_parser(i) {
        buf = lets;
    } else {
        return "Encountered error while attempting to change some \"let\" keywords to \"var\" "
            .to_string();
    }

    if let Ok((_, unis)) = uniform_vars_parser(&buf) {
        buf = unis;
    } else {
        return "Encountered error while attempting to add the root \"uni\" to the uniform variables."
            .to_string();
    }

    if let Ok((_, upda)) = replace_main_line(&buf) {
        buf = upda;
    } else {
        return "Encountered error while attempting to replace the mainImage(..) function."
            .to_string();
    }
    if let Ok((_, inout)) = replace_inouts(&buf) {
        buf = inout;
    } else {
        return "Encountered error while attempting to parse the inout storage qualifier and changing the corresponding variable to a pointer."
            .to_string();
    }

    if let Ok((_, voids)) = search_and_replace_void(&buf) {
        buf = voids;
    } else {
        return "Encountered error while attempting to erase all void symbols: \"-> ()\""
            .to_string();
    }

    if let Ok((_, tex)) = replace_all_texture_and_texel_fetch(&buf) {
        buf = tex;
    } else {
        return "Encountered error while attempting to replace all \"texture(..)\" and \"texelFetch(..)\" functions"
            .to_string();
    }

    if let Ok((_, tex)) = add_var_to_reassigned_args(&buf) {
        buf = tex;
    } else {
        return "Encountered error while attempting to insert var to args that are reassigned"
            .to_string();
    }

    buf
}

#[wasm_bindgen]
pub fn do_parse(x: String) -> String {
    if let Ok((_rest, replaced_defines)) = preprocessing(&x) {
        let trans = syntax::TranslationUnit::parse(Span::new(&replaced_defines));
        // println!("{:?}", trans);
        match trans {
            Err(err) => {
                let span = err.span();
                let fragment = *span.fragment();

                /////////////// begin formatting error message //////////////////////////////////////
                let buggy_line = if let Some(line) = fragment.lines().next() {
                    line
                } else {
                    "Error within error: there is no line to be parsed."
                }
                .to_string();

                let mut count = 0;
                let mut s = "".to_string();
                for c in buggy_line.chars() {
                    count += 1;
                    if count > 50 && c == ' ' {
                        s.push_str("\n\t");
                        count = 0;
                    }
                    s.push(c);
                }
                let mut intro = format!("There seems to be a syntax error in the input GLSL code: \nline: {:?}, column: {:?}, \nbuggy line:",
            span.location_line(), span.get_column(), ).to_string();
                intro.push_str(&s);
                /////////////// end formatting error message //////////////////////////////////////

                intro
            }

            Ok(w) => {
                let mut buf = String::new();

                transpiler::wgsl::show_translation_unit(&mut buf, &w);
                buf = postprocessing(&buf);

                return buf;
            }
        }
    } else {
        "Could not parse all the non-function #define lines".to_string()
    }
}
