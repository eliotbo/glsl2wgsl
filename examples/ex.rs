// use glsl::*;
use glsl2wgsl::do_parse;

use glsl2wgsl::parser::Parse;
// use glsl2wgsl::nom_helpers::Span;
// use glsl2wgsl::parser::Parse;

// use glsl2wgsl::let2var::let2var_parser;
// use glsl2wgsl::parse_func_defines::func_definition_parser;
use glsl2wgsl::insert_new_arg_vars::*;
use glsl2wgsl::parse_func_defines::*;
use glsl2wgsl::replace_defines::*;
use glsl2wgsl::replace_inouts::*;

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

const REASSIGNED_ARG: &str = "void func(vec2 fragColor, vec2 fragCoord)
{
    fragCoord = vec2(0.);
    fragColor = vec2(1.0);
}
";

const LET2VAR_SHORT: &str = "
float map(vec3 p) {
    float freq = SEA_FREQ;
    for(int i = 0; i < ITER_GEOMETRY; i++) {  
        h += d * amp;  
        uv *= octave_m; freq *= 1.9; amp *= 0.22;
    }
}
";

const LET2VAR_MANY_OCC: &str = "
float map(vec3 p) {
    float freq = 1.;
    d = freq;
    freq = 1.9; 
}
";

const VAR_DOT: &str = "
 float t = 1;
void main() {
  float atj = 4;
  atj.xy = vec2(5);
}
";

const MAT3: &str = "
 float d, h = 0;
";
// TODO: fix the newline for statements

// 2: delete all commented lines before going into the define_func
// 3: take all lines into account when reporting the line number in errors

fn main() {
    // //
    // // let replaced_defines_func = do_parse(TEXEL_FETCH.to_string());
    // // println!("{}", replaced_defines_func);

    // // // definition_parser(..) must be placed after func_definition_parser(..), because
    // // // the former erases all lines starting by #define
    // // let replaced_defines = definition_parser(&TEXEL_FETCH).unwrap().1;

    // let replaced_defines_func = func_definition_parser(&IN_OUT).unwrap().1;
    // let st =
    //     if let Ok((_rest, replaced_defines)) = definition_parser(replaced_defines_func.as_str()) {
    //         success(replaced_defines)("")
    //     } else {
    //         let vek = VerboseErrorKind::Context("Could not properly parse the #define functions");
    //         let _ve = VerboseError {
    //             errors: vec![("", vek)],
    //         };
    //         Err(nom::Err::Failure(_ve))
    //     }
    //     .unwrap()
    //     .1;

    // // if let Ok((a, b)) = st {
    // //     fs::write("./foo.txt", &replaced_defines_func).expect("Unable to write file");
    // // }

    // fs::write("./foo.txt", &replaced_defines_func).expect("Unable to write file");

    // // // println!("replaced_defines: {:?}", replaced_defines);

    // // To print the abstract syntax tree, uncomment the following line
    let trans = TranslationUnit::parse(Span::new(&MAT3)).unwrap();

    // let buf = do_parse(LET2VAR_SHORT.to_string());
    let buf = do_parse(MAT3.to_string());

    // let buf = replace_inouts(IN_OUT).unwrap().1;

    fs::write("./parsed_file.txt", &buf).expect("Unable to write file");

    println!("{:?}", trans);
    // println!("{:?}", buf);
}
