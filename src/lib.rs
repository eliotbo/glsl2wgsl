
#![allow(unsafe_code)]
use wasm_bindgen::prelude::*;
use parser::Parse;

#[cfg(test)]
mod parse_tests;
pub mod parser;
pub mod parsers;
pub mod syntax;
pub mod transpiler;
pub mod visitor;
pub mod let2var;

use let2var::let2var_parser;

#[wasm_bindgen]
extern {
    pub fn prompt(s: &str, o: &str) -> String;
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(v: &str)  {

}

#[wasm_bindgen]
pub fn do_parse(x: String) -> String {
    let trans = syntax::TranslationUnit::parse(&x).unwrap();
    let mut buf = String::new();
    
    transpiler::wgsl::show_translation_unit(&mut buf, &trans);
    buf = let2var_parser(&buf).unwrap().1;
    return buf
  }

