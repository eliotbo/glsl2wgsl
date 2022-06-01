// use glsl2wgsl::parser::Parse;
use glsl2wgsl::parser::Parse;
use glsl2wgsl::nom_helpers::Span;
use glsl2wgsl::syntax;
// use glsl::*;
use glsl2wgsl::transpiler::wgsl::show_translation_unit;
use glsl2wgsl::let2var::let2var_parser;
use glsl2wgsl::replace_unis::uniform_vars_parser;

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

const TEST1: &str = "

mat3 getRotZMat(float a){return mat3(cos(a),-sin(a),0.,sin(a),cos(a),0.,0.,0.,1.);}

float dstepf = 0.0;

float map(vec3 p)
{
	p.x += sin(p.z*1.8);
    p.y += cos(p.z*.2) * sin(p.x*.8);
	p *= getRotZMat(p.z*0.8+sin(p.x)+cos(p.y));
    p.xy = mod(p.xy, 0.3) - 0.15;
	dstepf += 0.003;
	return length(p.xy);
}

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
	vec2 uv = (fragCoord - iResolution.xy*.5 )/iResolution.y;
    vec3 rd = normalize(vec3(uv, (1.-dot(uv, uv)*.5)*.5)); 
    vec3 ro = vec3(0, 0, iTime*1.26), col = vec3(0), sp;
	float cs = cos( iTime*0.375 ), si = sin( iTime*0.375 );    
    rd.xz = mat2(cs, si,-si, cs)*rd.xz;
	float t=0.06, layers=0., d=0., aD;
    float thD = 0.02;
	for(float i=0.; i<250.; i++)	
	{
        if(layers>15. || col.x > 1. || t>5.6) break;
        sp = ro + rd*t;
        d = map(sp); 
        aD = (thD-abs(d)*15./16.)/thD;
        if(aD>0.) 
		{ 
            col += aD*aD*(3.-2.*aD)/(1. + t*t*0.25)*.2; 
            layers++; 
		}
        t += max(d*.7, thD*1.5) * dstepf; 
	}
    col = max(col, 0.);
    col = mix(col, vec3(min(col.x*1.5, 1.), pow(col.x, 2.5), pow(col.x, 12.)), 
              dot(sin(rd.yzx*8. + sin(rd.zxy*8.)), vec3(.1666))+0.4);
    col = mix(col, vec3(col.x*col.x*.85, col.x, col.x*col.x*0.3), 
             dot(sin(rd.yzx*4. + sin(rd.zxy*4.)), vec3(.1666))+0.25);
	fragColor = vec4( clamp(col, 0., 1.), 1.0 );
}
";

const TWO_FN: &str = "
void main() {}
void main() {}
";

const COND: &str = "
void main() {
  if (w) {
    if (w) {
      return true;
    }
  } else {
    return false;
  }
}";

const PAINT: &str ="
void mainImage( out vec4 U, in vec2 pos )
{
    if (u.y < 1.2)
    {
      float u = iRes;
        for (float y = 0.; y > -3.; y--)
          {
            float u =  45;
            for (float x = -2.; x <3.; x++)
            {
                id = floor(u) + vec2(x,y);
                lc = (fract(u) + vec2(1.-x,-y))/vec2(5,3);
                h = (hash12(id)-.5)*.25+.5; //shade and length for an individual blade of grass

                lc-= vec2(.3,.5-h*.4);
                lc.x += sin(((iTime*1.7+h*2.-id.x*.05-id.y*.05)*1.1+id.y*.5)*2.)*(lc.y+.5)*.5;
                t = abs(lc)-vec2(.02,.5-h*.5);
                l =  length(max(t,0.)) + min(max(t.x,t.y),0.); //distance to the segment (blade of grass)

                l -= noise (lc*7.+id)*.1; //add fine noise
                C = vec4(f*.25,st(l,.1,sm*xd*.09)); //grass outline                
                C = mix(C,vec4(f                  //grass foregroud
                            *(1.2+lc.y*2.)  //the grass is a little darker at the root
                            *(1.8-h*2.5),1.)    //brightness variations for individual blades of grass
                            ,st(l,.04,sm*xd*.09));
                
                O = mix (O,C,C.a*step (id.y,-1.));
                a = max (a, C.a*step (id.y,-5.));  //a mask to cover the trunk of the tree with grasses in the foreground
            }
        }
    }
}
";

fn main() {
  // let r = TEST1;
  let r = PAINT;

  let trans = syntax::TranslationUnit::parse(Span::new(r)).unwrap();
  let mut buf = String::new();
  
  show_translation_unit(&mut buf, &trans);
  buf = let2var_parser(&buf).unwrap().1;
  buf = uniform_vars_parser(&buf).unwrap().1;
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
  let trans = syntax::TranslationUnit::parse(Span::new(x)).unwrap();
  let mut buf = String::new();
  
  show_translation_unit(&mut buf, &trans);
  return buf
  // fs::write("./foo.txt", &buf).expect("Unable to write file");
}
