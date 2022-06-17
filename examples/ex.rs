// use glsl::*;
use glsl2wgsl::do_parse;

use glsl2wgsl::parser::Parse;
// use glsl2wgsl::nom_helpers::Span;
// use glsl2wgsl::parser::Parse;

// use glsl2wgsl::let2var::let2var_parser;
// use glsl2wgsl::parse_func_defines::func_definition_parser;
use glsl2wgsl::parse_func_defines::*;
use glsl2wgsl::replace_defines::*;

// use glsl2wgsl::replace_inouts::{replace_inouts, search_and_replace_void};
// use glsl2wgsl::replace_main::replace_main_line;
// use glsl2wgsl::replace_texel_fetch::*;
// use glsl2wgsl::replace_unis::uniform_vars_parser;

// use glsl2wgsl::syntax::TranslationUnit;
// use glsl2wgsl::parser;
use glsl2wgsl::parsers_span::Span;
use glsl2wgsl::syntax::TranslationUnit;
use glsl2wgsl::transpiler::wgsl::show_translation_unit;

use nom::combinator::success;
use nom::error::{VerboseError, VerboseErrorKind};
use std::fs;

// TODO: fix the newline for statements
// 1: in and out keywords in function arguments
// 2: delete all commented lines before going into the define_func

const DEFINE_FUNC_COMMA: &str = "void norm(vec3 po) {
  if (r.x > d.x)   t =3;
  
}";

fn main() {
    //
    // let replaced_defines_func = do_parse(TEXEL_FETCH.to_string());
    // println!("{}", replaced_defines_func);

    // // definition_parser(..) must be placed after func_definition_parser(..), because
    // // the former erases all lines starting by #define
    // let replaced_defines = definition_parser(&TEXEL_FETCH).unwrap().1;

    let replaced_defines_func = func_definition_parser(&DEFINE_FUNC_COMMA).unwrap().1;
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

    let trans = TranslationUnit::parse(Span::new(&st)).unwrap();

    // let buf = do_parse(ALL.to_string());
    let buf = do_parse(DEFINE_FUNC_COMMA.to_string());

    fs::write("./parsed_file.txt", &buf).expect("Unable to write file");

    println!("{:?}", trans);
    // println!("{:?}", buf);
}
