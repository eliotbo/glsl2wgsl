use glsl2wgsl::do_parse;
use std::fs;

use glsl2wgsl::parse_func_defines::*;
use glsl2wgsl::replace_defines::*;

fn main() {
    let file_content =
        fs::read_to_string("examples/glsl_file_to_parse.glsl").expect("couldn't read file");

    let replaced = func_definition_parser(&file_content).unwrap().1;
    let replaced2 = definition_parser(&replaced).unwrap().1;
    fs::write("./foo.txt", &replaced2).expect("Unable to write file");

    let output = do_parse(file_content);
    fs::write("./parsed_file.txt", &output).expect("Unable to write file");
}
