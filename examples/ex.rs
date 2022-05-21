use glsl2wgsl::parser::Parse;
use glsl2wgsl::syntax;
// use glsl::*;
use glsl2wgsl::transpiler::wgsl::show_translation_unit;
use glsl2wgsl::let2var::let2var_parser;

use std::fs;

const ALL: &str = 
"vec2 e = vec2(3.0);
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

const SIMPLE_VEC2: &str = 
"vec2 e = vec2(3.0);
float b = 1.0;";

const CONST: &str = "const vec2 e = vec2(.00035, -.00035);";
const FUNC_PROTO: &str = "
vec3 norm(vec3 po) {}
vec2 norm2(vec2 wq) {}
";
const FUNC_PROTO_CONTENT: &str =    
 "vec3 norm(vec3 po) {
  int what = 3;
  int a = 2;
  return what;
}";
const ASSIGN_OP: &str =     
  "vec4 rd = vec4(0.);
void norm(vec3 po) {
  rd.x *= 2. + 3.;
}";
const ASSIGN_OP_WGSL: &str  = 
"let rd: vec4<f32> = vec4<f32>(0.);\nnorm(po: vec3<f32>) -> () {\n\trd.x = rd.x * (2.+3.);\n}\n\n";

const FOR_LOOP: &str = "void main() { for(int i = 0; i < 120; i++) { a = 3; } }";
const ARRAYED_DECLARATION: &str = 
"void norm(vec3 po) {
  float r = 2.0, e = 1.0;
}";

const IF_ELSE: &str = 
"void norm(vec3 po) {
  if (r.x > d.x) {
    r = d;
  } 
  else {
    r = 1.0;
    a = 55;
  }

}";

const IF: &str = 
"void norm(vec3 po) {
  if (r.x > d.x) r = d;
}";

const IN_OUT: &str = "void mainImage( out vec4 fragColor, in vec2 fragCoord )
{}";

const SIMPLE_STRUCT: &str =   
"struct Light
{
float intensity;
};
";

const STRUCT: &str =   
"struct Light
{
float intensity;
vec3 position;
};
uniform Light myLights;
";

const ARRAY: &str = 
"const float yaa[2] = float[2](5.5, 8.7);";

fn main() {
  let r = ALL;
  // let r = SIMPLE_STRUCT;

  let trans = syntax::TranslationUnit::parse(r).unwrap();
  let mut buf = String::new();
  
  show_translation_unit(&mut buf, &trans);
  buf = let2var_parser(&buf).unwrap().1;
  fs::write("./foo.txt", &buf).expect("Unable to write file");
  
  println!("{:?}", trans);
  println!("{:?}", buf);

  // assert_eq!(&do_parse(SIMPLE_VEC2), "let e: vec2<f32> = vec2<f32>(3.);\nlet b: f32 = 1.;\n");
  // assert_eq!(&do_parse(CONST), "const e: vec2<f32> = vec2<f32>(0.00035, -0.00035);\n");
  // assert_eq!(&do_parse(FUNC_PROTO), "norm(po: vec3<f32>) -> vec3<f32> {\n}\n\nnorm2(wq: vec2<f32>) -> vec2<f32> {\n}\n\n");
  // assert_eq!(&do_parse(FUNC_PROTO_CONTENT),"norm(po: vec3<f32>) -> vec3<f32> {\n\tlet what: i32 = 3;\n\tlet a: i32 = 2;\n\treturn what;\n}\n\n");
  // assert_eq!(&do_parse(ASSIGN_OP), ASSIGN_OP_WGSL);
  // assert_eq!(&do_parse(FOR_LOOP), "main() -> () {\n\tfor (let i: i32 = 0; i<120; i = i + 1) {\n\t\ta = 3;\n\t}\n}\n\n");
  // assert_eq!(&do_parse(ARRAYED_DECLARATION), "norm(po: vec3<f32>) -> () {\n\tlet r: f32 = 2.;\n\tlet e: f32 = 1.;\n}\n\n");
  // assert_eq!(&do_parse(MULTI_DECLARATION), "norm(po: vec3<f32>) -> () {\n    if (r.x>d.x) {\nr = d;\n}\n}\n");
}

fn do_parse(x: &str) -> String {
  let trans = syntax::TranslationUnit::parse(x).unwrap();
  let mut buf = String::new();
  
  show_translation_unit(&mut buf, &trans);
  return buf
  // fs::write("./foo.txt", &buf).expect("Unable to write file");
}
