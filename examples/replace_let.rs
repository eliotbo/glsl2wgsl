use glsl2wgsl::parser::Parse;
use glsl2wgsl::syntax;
// use glsl::*;
use glsl2wgsl::do_parse;
use glsl2wgsl::let2var::*;
use glsl2wgsl::transpiler::wgsl::show_translation_unit;

use glsl2wgsl::parsers_span::nom_helpers::Span;

use nom::combinator::peek;

use std::fs;

// const TEST1: &str = "
// float yu = 1;
// float sa = 1;
// ";

const TEST1: &str = "
vec2 a = vec2(1,1);
afdsf kldsgjdsflkj fdslkklsjg fdkl dfjgklfs;gjfds gfdjg fdjgksdflj gslkfdjgdkl gfdlkjg dflkgjdf glfdjk
";

fn main() {
    println!("{:?}", do_parse(TEST1.to_string()));

    // println!("{:?}", blank_space_span(Span::new(TEST1)));

    // println!("{:?}", read_type(": type = 12;"));
    // println!("{:?}", read_type(" = 12;"));

    // println!("{:?}", either_type_or_not(": type = 12;"));
    // println!("{:?}", either_type_or_not(" = 12;"));

    // println!("{:?}", read_named_var("let name: type = 12;"));
    // println!("{:?}", read_named_var("let name = 12;"));

    // println!("{:?}", is_repeated("name = ")("
    //     name2 = 33;
    //     let bah: f32;
    //     name3 = 34;
    //     "));
    // println!("decl_is_reassigned: {:?}", write_var_or_let("sss
    //     let name: type = 12;
    //     let bah: f32;
    //     name2 = 34;
    //     "));
    // println!("{:?}", write_var_or_let("let name: type = 12;
    //     let bah: f32;
    //     name = 34;
    //     "));
    // println!("{:?}", replace_1_let("
    //     wha = 4;
    //     let name: type = 12;
    //     let bah: f32;
    //     name = 34;
    //     "));
    // println!("{:?}", replace_all_let("
    //     wha = 4;
    //     let name: type = 12;
    //     let bah: f32;
    //     name = 34;
    //     "));

    // println!("let2var_parser: {:?}", let2var_parser("
    //     wha = 4;
    //     let name: type = 12;
    //     let bah: f32 = 55;
    //     go = 55;
    //     bah = 33;
    //     name = 34;
    //     go = 22;

    //     "));
}

// Ok(TranslationUnit(
//     NonEmpty([
//         FunctionDefinition(
//             FunctionDefinition {
//                  prototype: FunctionPrototype { ty: FullySpecifiedType { qualifier: None, ty: TypeSpecifier {
//                      ty: Void, array_specifier: None } }, name: Identifier("mainImage"), parameters: [] },
//                     statement: CompoundStatement { statement_list: [] } })
//         , FunctionDefinition(FunctionDefinition { prototype: FunctionPrototype { ty: FullySpecifiedType { qualifier: None, ty: TypeSpecifier { ty: Void, array_specifier: None } }, name: Identifier("a"), parameters: [] }, statement: CompoundStatement { statement_list: [] } })])))
