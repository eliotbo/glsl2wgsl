use glsl2wgsl::parser::Parse;
use glsl2wgsl::syntax;
// use glsl::*;
use glsl2wgsl::transpiler::wgsl::show_translation_unit;
use glsl2wgsl::let2var::*;
use nom::combinator::peek;

use std::fs;

fn main() {
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

    println!("let2var_parser: {:?}", let2var_parser("
        wha = 4;
        let name: type = 12;
        let bah: f32 = 55;
        go = 55;
        bah = 33;
        name = 34;
        go = 22;

        "));
}
