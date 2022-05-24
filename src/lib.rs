#![allow(unsafe_code)]
use parser::Parse;
use wasm_bindgen::prelude::*;

pub mod let2var;

#[cfg(test)]
mod parse_tests;
pub mod parser;
pub mod parsers;
pub mod parsers_span;

pub mod syntax;
pub mod transpiler;

use let2var::let2var_parser;

#[wasm_bindgen]
extern "C" {
    pub fn prompt(s: &str, o: &str) -> String;
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(v: &str) {}

#[wasm_bindgen]
pub fn do_parse(x: String) -> String {
    let trans = syntax::TranslationUnit::parse(&x);
    println!("{:?}", trans);
    match trans {
        Err(err) => return format!("There is a syntax error in the input GLSL code: {:?}", err),
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
