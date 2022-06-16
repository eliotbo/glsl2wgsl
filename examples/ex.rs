// use glsl2wgsl::parser::Parse;
use glsl2wgsl::nom_helpers::Span;
use glsl2wgsl::parser::Parse;
use glsl2wgsl::syntax;
// use glsl::*;
use glsl2wgsl::let2var::let2var_parser;
use glsl2wgsl::parse_func_defines::func_definition_parser;
use glsl2wgsl::replace_defines::definition_parser;
use glsl2wgsl::replace_inouts::{replace_inouts, search_and_replace_void};
use glsl2wgsl::replace_main::replace_main_line;
use glsl2wgsl::replace_texel_fetch::*;
use glsl2wgsl::replace_unis::uniform_vars_parser;
use glsl2wgsl::transpiler::wgsl::show_translation_unit;

use std::fs;

#[allow(dead_code)]
const ALL: &str = "vec2 e = vec2(3.0);
float b = 1.0;
vec3 norm(vec3 po) {}
vec2 norm2(vec2 wq) {}
vec3 norm(vec3 po) {
  int what = 3;
  int a = 2;
  return what;
}
vec4 rd = vec4(0.);
void norm(vec3 po) {
  rd.x *= 2. + 3.;
}
vec4 rd = vec4(0.);
void norm(vec3 po) {
  rd.x *= 2. + 3.;
}
void main() { for(int i = 0; i < 120; i++) { a = 3; } }

void norm(vec3 po) {
  float r = 2.0, e = 1.0;
}

void norm(vec3 po) {
  if (r.x > d.x) r = d;
}

void norm(vec3 po) {
  if (r.x > d.x) {
    r = d;
  } 
  else {
    r = 1.0;
    a = 55;
  }
  col += gl;
}

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{}

struct Light
{
  float intensity;
  vec3 position;
};

uniform Light myLights;

const float yaa[1] = float[1](5.5);

const float yaa[2] = float[2](5.5, 8.7);

";

// TODO: fix the newline for statements
// 1: in and out keywords in function arguments
// THEN: big clean

fn main() {
    let replaced_defines_func: String;

    replaced_defines_func = func_definition_parser(&ALL).unwrap().1;

    // println!("replaced_defines_func: {:?}", replaced_defines_func);

    let trans = syntax::TranslationUnit::parse(Span::new(&replaced_defines_func)).unwrap();

    let mut buf: String = String::new();
    show_translation_unit(&mut buf, &trans);

    buf = let2var_parser(&buf).unwrap().1;
    buf = uniform_vars_parser(&buf).unwrap().1;
    buf = definition_parser(&buf).unwrap().1;
    buf = replace_main_line(&buf).unwrap().1;
    buf = replace_inouts(&buf).unwrap().1;
    buf = search_and_replace_void(&buf).unwrap().1;
    buf = replace_all_texture_and_texel_fetch(&buf).unwrap().1;

    fs::write("./foo.txt", &buf).expect("Unable to write file");

    println!("{:?}", trans);
    // println!("{:?}", buf);
}
