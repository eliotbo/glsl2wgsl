// use glsl::*;
use glsl2wgsl::do_parse;

// use glsl2wgsl::parser::Parse;
// use glsl2wgsl::nom_helpers::Span;
// use glsl2wgsl::parser::Parse;
// use glsl2wgsl::syntax;
// use glsl2wgsl::let2var::let2var_parser;
// use glsl2wgsl::parse_func_defines::func_definition_parser;
use glsl2wgsl::parse_func_defines::*;
use glsl2wgsl::replace_defines::*;

// use glsl2wgsl::replace_inouts::{replace_inouts, search_and_replace_void};
// use glsl2wgsl::replace_main::replace_main_line;
// use glsl2wgsl::replace_texel_fetch::*;
// use glsl2wgsl::replace_unis::uniform_vars_parser;
// use glsl2wgsl::transpiler::wgsl::show_translation_unit;

use nom::combinator::success;
use nom::error::{VerboseError, VerboseErrorKind};
use std::fs;

// TODO: fix the newline for statements
// 1: in and out keywords in function arguments
// 2: delete all commented lines before going into the define_func

const TEXEL_FETCH: &str = "
#define _sub   S(45);  
#define S(a) c+=char(a);  tp.x-=FONT_SPACE;


void main() {
  
    float c = 0.;
  if (value < 0) 
  { value = -value;
    if (minDigits < 1) minDigits = 1;
    else minDigits--;
    _sub                   // add minus char
  } 
  
  for (int ni=0; ni<10; ni++)
  {
    fn /= 10;
    if (fn == 0) break;
    digits++;
  } 
}

void f() {
  float aaa = bcbx, cxvb = 1;
}
";
fn main() {
    //
    // let replaced_defines_func = do_parse(TEXEL_FETCH.to_string());
    // println!("{}", replaced_defines_func);

    // // definition_parser(..) must be placed after func_definition_parser(..), because
    // // the former erases all lines starting by #define
    // let replaced_defines = definition_parser(&TEXEL_FETCH).unwrap().1;

    let replaced_defines_func = func_definition_parser(&TEXEL_FETCH).unwrap().1;
    let st =
        if let Ok((_rest, replaced_defines)) = definition_parser(replaced_defines_func.as_str()) {
            success(replaced_defines)("")
        } else {
            let vek = VerboseErrorKind::Context("Could not properly parse the #define functions");
            let _ve = VerboseError {
                errors: vec![("", vek)],
            };
            Err(nom::Err::Failure(_ve))
        }
        .unwrap()
        .1;

    // if let Ok((a, b)) = st {
    //     fs::write("./foo.txt", &replaced_defines_func).expect("Unable to write file");
    // }

    fs::write("./foo.txt", &replaced_defines_func).expect("Unable to write file");

    // // println!("replaced_defines: {:?}", replaced_defines);

    // let trans = syntax::TranslationUnit::parse(Span::new(&replaced_defines)).unwrap();
    // .1;

    // let buf = do_parse(ALL.to_string());
    let buf = do_parse(TEXEL_FETCH.to_string());

    fs::write("./parsed_file.txt", &buf).expect("Unable to write file");

    // println!("{:?}", trans);
    // println!("{:?}", buf);
}
