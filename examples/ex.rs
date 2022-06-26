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
use glsl2wgsl::replace_main::*;
use glsl2wgsl::replace_mod::*;

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

// 2: delete all commented lines before going into the define_func
// 3: take all lines into account when reporting the line number in errors
// 5. clamp(sum, 0., 1.); // where  sum is a vec2
// 7. mod(a, b)
// fn fbm(uv: vec2<f32>) -> f32 {
// 	let f: f32;
// 	let m: mat2x2<f32> = mat2x2<f32>(1.6, 1.2, -1.2, 1.6);
// 	f = 0.5 * noise(uv);
// 	uv = m * uv;
// 	f = f + (0.25 * noise(uv));
// 	uv = m * uv;
// 	f = f + (0.125 * noise(uv));
// 	uv = m * uv;
// 	f = f + (0.0625 * noise(uv));
// 	uv = m * uv;
// 	f = 0.5 + 0.5 * f;
// 	return f;
// }

// 8. inverseSqrt

const ONE_MOD: &str = "mod(g, q);
a + mod(asdfas, rtefg(dd));";

const MOD: &str = "void main()  {
    float a = mod(g, q);
    float b = mod(mod(qr, to), mod(other, less));
} 
";

fn main() {
    // // // println!("replaced_defines: {:?}", replaced_defines);

    // // To print the abstract syntax tree, uncomment the following line
    // let trans = TranslationUnit::parse(Span::new(&MAT3)).unwrap();

    // let buf = do_parse(LET2VAR_SHORT.to_string());
    let buf = do_parse(MOD.to_string());

    // let buf = parse_var_and_type(MAIN_WGSL).unwrap().1;
    // let ret = replace_all_mods(MOD);
    // println!("{:?}", ret);
    // let buf = ret.unwrap().1;

    fs::write("./parsed_file.txt", &buf).expect("Unable to write file");

    // println!("{:?}", trans);
    // println!("{:?}", buf);
}

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
