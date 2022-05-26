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

use let2var::let2var_parser;
use parsers_span::Span;

#[wasm_bindgen]
extern "C" {
    pub fn prompt(s: &str, o: &str) -> String;
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(v: &str) {}

#[wasm_bindgen]
pub fn do_parse(x: String) -> String {
    let trans = syntax::TranslationUnit::parse(Span::new(&x));
    println!("{:?}", trans);
    match trans {
        Err(err) => {
            let span = err.span();
            let fragment = *span.fragment();
            // let offset = span.location_offset();
            // let problematic_code_to_end = &fragment[offset..];
            let buggy_line = if let Some(line) = fragment.lines().next() {
                line
            } else {
                "Error within error: there is no line to be checked."
            };
            let err_long_str =
            format!("There is a syntax error in the input GLSL code: \nline: {:?}, column: {:?}, \nbuggy line: {}", 
                span.location_line(), span.get_column(), buggy_line);

            let mut count = 0;
            let mut s = "".to_string();
            for c in err_long_str.chars() {
                count += 1;
                if count > 60 && c == ' ' {
                    s.push('\n');
                    count = 0;
                }
                s.push(c);
            }
            s
        }
        Ok(w) => {
            let mut buf = String::new();

            transpiler::wgsl::show_translation_unit(&mut buf, &w);

            let wha = let2var_parser(&buf);
            println!("{:?}", wha);
            buf = wha.unwrap().1;

            return buf;
        }
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
