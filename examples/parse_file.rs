use glsl2wgsl::do_parse;
use std::fs;

fn main() {
    let file_content =
        fs::read_to_string("examples/glsl_file_to_parse.glsl").expect("couldn't read file");
    let output = do_parse(file_content);
    fs::write("./foo.txt", &output).expect("Unable to write file");
}
