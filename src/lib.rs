#![allow(unsafe_code)]
use parser::Parse;
use wasm_bindgen::prelude::*;

pub mod let2var;

#[cfg(test)]
mod parse_tests;
pub mod parser;
// pub mod parsers;
pub mod parsers_span;

pub mod syntax;
pub mod transpiler;

pub mod nom_helpers;
pub mod parse_func_defines;
pub mod replace_defines;
pub mod replace_inouts;
pub mod replace_main;
pub mod replace_unis;

use parse_func_defines::*;
use replace_defines::*;
use replace_inouts::*;
use replace_unis::*;

use let2var::let2var_parser;
use parsers_span::Span;
use replace_main::replace_main_line;

#[wasm_bindgen]
extern "C" {
    pub fn prompt(s: &str, o: &str) -> String;
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(v: &str) {}

#[wasm_bindgen]
pub fn do_parse(x: String) -> String {
    let replaced_defines_func: String;
    if let Ok((_rest, replaced_defines_func)) = func_definition_parser(&x) {
        let trans = syntax::TranslationUnit::parse(Span::new(&replaced_defines_func));
        println!("{:?}", trans);
        match trans {
            Err(err) => {
                let span = err.span();
                let fragment = *span.fragment();
                // let offset = span.location_offset();
                // let problematic_code_to_end = &fragment[offset..];

                /////////////// begin formatting error message //////////////////////////////////////
                let mut buggy_line = if let Some(line) = fragment.lines().next() {
                    line
                } else {
                    "Error within error: there is no line to be checked."
                }
                .to_string();
                // let err_long_str =
                // format!("There seems to be a syntax error in the input GLSL code: \nline: {:?}, column: {:?}, \nbuggy line: {}",
                //     span.location_line(), span.get_column(), buggy_line);

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

                // the following parsers cannot fail, so we can use unwrap freely
                let lets = let2var_parser(&buf).unwrap();
                let unis = uniform_vars_parser(&lets.1).unwrap();
                let defi = definition_parser(&unis.1).unwrap().1;
                let upda = replace_main_line(&defi).unwrap().1;
                let inout = replace_inouts(&upda).unwrap().1;

                // show_translation_unit(&mut buf, &trans);
                // buf = let2var_parser(&buf).unwrap().1;
                // buf = uniform_vars_parser(&buf).unwrap().1;
                // buf = definition_parser(&buf).unwrap().1;
                // buf = replace_main_line(&buf).unwrap().1;

                // let private_vars = add_private_to_global_vars(&defi);

                println!("{:?}", inout);
                buf = inout;

                return buf;
            }
        }
    } else {
        "Could not convert a function(s) with the \"#define\" keyword".to_string()
    }
}

// Ok(
//     TranslationUnit(
//         NonEmpty([
//             Declaration(
//                 InitDeclaratorList(
//                     InitDeclaratorList {
//                         head: SingleDeclaration {
//                             ty: FullySpecifiedType { qualifier: None, ty: TypeSpecifier { ty: Float, array_specifier: None } },
//                             name: Some(Identifier("yu")), array_specifier: None, initializer: Some(Simple(IntConst(1)))
//                         },
//                         tail: [] }))])))

// Ok(
//     TranslationUnit(
//         NonEmpty([
//             Declaration(
//                 InitDeclaratorList(
//                     InitDeclaratorList {
//                         head: SingleDeclaration {
//                             ty: FullySpecifiedType { qualifier: None, ty: TypeSpecifier { ty: Float, array_specifier: None } },
//                             name: Some(Identifier("yu")), array_specifier: None, initializer: Some(Simple(IntConst(1))) }, tail: [] })),
//             Declaration(InitDeclaratorList(
//                 InitDeclaratorList {
//                     head: SingleDeclaration {
//                         ty: FullySpecifiedType {
//                             qualifier: None, ty: TypeSpecifier { ty: Float, array_specifier: None } }, name: Some(Identifier("sa")),
//                             array_specifier: None, initializer: Some(Simple(IntConst(1))) }, tail: [] }))])))
