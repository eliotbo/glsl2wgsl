use crate::do_parse;

#[test]
fn replace_main() {
    const MAIN_IMAGE: &str = "void mainImage( out vec4 fragColor, in vec2 fragCoord ) {}";

    let b = "[[stage(compute), workgroup_size(8, 8, 1)]]
fn update([[builtin(global_invocation_id)]] invocation_id: vec3<u32>) {
    let R: vec2<f32> = uni.iResolution.xy;
    let y_inverted_location = vec2<i32>(i32(invocation_id.x), i32(R.y) - i32(invocation_id.y));
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    
	var fragColor: vec4<f32>;
	var fragCoord = vec2<f32>(f32(location.x), f32(location.y) );

} 

";

    assert_eq!(&do_parse(MAIN_IMAGE.to_string()), b);
}

#[test]
fn single_line_if() {
    let a: &str = "void norm(vec3 po) {
  if (r.x > d.x)   t =3;
  
}";

    let b = "fn norm(po: vec3<f32>)  {
	if (r.x > d.x) { t = 3; }
} 

";

    assert_eq!(&do_parse(a.to_string()), b);
}

#[test]
fn two_lines_if() {
    let a: &str = "void norm(vec3 po) {
  if (r.x > d.x)  { 
    r = d;
    t =3;
  }
} ";

    let b = "fn norm(po: vec3<f32>)  {
	if (r.x > d.x) {
		r = d;
		t = 3;
	}
} \n\n";

    assert_eq!(&do_parse(a.to_string()), b);
}

#[test]
fn simple_vec2() {
    let a: &str = "vec2 e = vec2(3.0);
float b = 1.0;";

    let b = "let e: vec2<f32> = vec2<f32>(3.);
let b: f32 = 1.;
";

    assert_eq!(&do_parse(a.to_string()), b);
}

// #[test]
// fn simple_const() {
//     let a: &str =
// "const vec2 e = vec2(.00035, -.00035);";

//     let b =
// "const e: vec2<f32> = vec2<f32>(0.00035, -0.00035);
// ";

//     assert_eq!(&do_parse(a.to_string()), b);
// }

#[test]
fn func_proto() {
    const FUNC_PROTO: &str = "
vec3 norm(vec3 po) {}
vec2 norm2(vec2 wq) {}
";

    let b = "fn norm(po: vec3<f32>) -> vec3<f32> {
} 

fn norm2(wq: vec2<f32>) -> vec2<f32> {
} 

";

    assert_eq!(&do_parse(FUNC_PROTO.to_string()), b);
}

#[test]
fn func_proto_content() {
    const FUNC_PROTO_CONTENT: &str = "vec3 norm(vec3 po) {
  int what = 3;
  int a = 2;
  return what;
}";

    let b = "fn norm(po: vec3<f32>) -> vec3<f32> {
	let what: i32 = 3;
	let a: i32 = 2;
	return what;
} 

";

    assert_eq!(&do_parse(FUNC_PROTO_CONTENT.to_string()), b);
}

#[test]
fn assign_op() {
    const ASSIGN_OP: &str = "vec4 rd = vec4(0.);
void norm(vec3 po) {
  rd.x *= 2. + 3.;
}";

    let b = "var rd: vec4<f32> = vec4<f32>(0.);
fn norm(po: vec3<f32>)  {
	rd.x = rd.x * (2. + 3.);
} 

";

    assert_eq!(&do_parse(ASSIGN_OP.to_string()), b);
}

#[test]
fn for_loop() {
    const FOR_LOOP: &str = "void main() { for(int i = 0; i < 120; i++) { a = 3; } }";
    let b = "fn main()  {

	for (var i: i32 = 0; i < 120; i = i + 1) {
		a = 3;
	}

} 

";

    assert_eq!(&do_parse(FOR_LOOP.to_string()), b);
}

#[test]
fn array_decl1() {
    const ARRAYED_DECLARATION: &str = "void norm(vec3 po) {
  float r = 2.0, e = 1.0;
}";

    let b = "fn norm(po: vec3<f32>)  {
	let r: f32 = 2.;
	let e: f32 = 1.;
} 

";

    assert_eq!(&do_parse(ARRAYED_DECLARATION.to_string()), b);
}

#[test]
fn array_decl2() {
    const ARRAYED_DECLARATION: &str = " float d = 0.0, h;";

    let b = "let d: f32 = 0.;
let h: f32 = 0.;
";

    assert_eq!(&do_parse(ARRAYED_DECLARATION.to_string()), b);
}

#[test]
fn array_decl3() {
    const ARRAYED_DECLARATION: &str = " float d, h = 0;";

    let b = "var<private> d: f32 = 0.;
let h: f32 = 0.;
";

    assert_eq!(&do_parse(ARRAYED_DECLARATION.to_string()), b);
}

#[test]
fn if_else() {
    const IF_ELSE: &str = "void norm(vec3 po) {
  if (r.x > d.x) {
    r = d;
  } 
  else {
    r = 1.0;
    a = 55;
  }

}";

    let b = "fn norm(po: vec3<f32>)  {
	if (r.x > d.x) {
		r = d;
	} else { 
		r = 1.;
		a = 55;
	}
} 

";

    assert_eq!(&do_parse(IF_ELSE.to_string()), b);
}

// #[test]
// fn inout() {
//     const IN_OUT: &str = "void func( out vec4 fragColor, in vec2 fragCoord )
// {}";

//     let b =
// "fn func(fragColor: vec4<f32>, fragCoord: vec2<f32>)  {
// }

// ";

//     assert_eq!(&do_parse(IN_OUT.to_string()), b);
// }

#[test]
fn just_out() {
    const OUT: &str = "void func( out vec4 fragColor, in vec2 fragCoord )
{ fragColor = vec2(0); }";

    let b = "fn func(fragColor: ptr<function, vec4<f32>>, fragCoord: vec2<f32>)  {
	(*fragColor) = vec2<f32>(0.);
} 

";

    assert_eq!(&do_parse(OUT.to_string()), b);
}

#[test]
fn simple_struct() {
    const SIMPLE_STRUCT: &str = "struct Light
{
float intensity;
};
";
    let b = "struct Light {
	intensity: f32;
};
";

    assert_eq!(&do_parse(SIMPLE_STRUCT.to_string()), b);
}

#[test]
fn array() {
    const CALLED_ARRAY: &str = "fn main() {  f32 a[5] = b[3]; }";

    let b = "fn main() -> fn {
	let a: array<f32,5> = b[3];
} 

";

    assert_eq!(&do_parse(CALLED_ARRAY.to_string()), b);
}

#[test]
fn array_assignment() {
    const MAT3: &str = "
void dum() {
    mat3 m;
    m[0] = vec3(1.0);
}
";

    let b = "fn dum()  {
	var m: mat3x3<f32>;
	m[0] = vec3<f32>(1.);
} 

";

    assert_eq!(&do_parse(MAT3.to_string()), b);
}

#[test]
fn two_fn() {
    const TWO_FN: &str = "
void main() {}
void main() {}
";

    let b = "fn main()  {
} 

fn main()  {
} 

";

    assert_eq!(&do_parse(TWO_FN.to_string()), b);
}

#[test]
fn cond() {
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

    let b = "fn main()  {
	if (w) {
		if (w) {
			return true;
		}
	} else { 
		return false;
	}
} 

";

    assert_eq!(&do_parse(COND.to_string()), b);
}

#[test]
fn defs() {
    const DEFS: &str = "#define SOME
// #define OTHER
";

    let b = "let SOME: bool = true;
let OTHER: bool = false;
";

    assert_eq!(&do_parse(DEFS.to_string()), b);
}

#[test]
fn ifdefs() {
    const IFDEFS: &str = " 
void main() {
    #ifdef SOME_VAR
        t = 1;
    #else 
        t = 2;
    #endif
}";

    let b = "fn main()  {
	if (SOME_VAR) {
		t = 1;
	} else { 
		t = 2;
	}
} 

";

    assert_eq!(&do_parse(IFDEFS.to_string()), b);
}

#[test]
fn define() {
    const DEFINE: &str = "
#define in_body 10.0
#define no_in_body 3.4

float func(float x) {
  q = in_body;
}
";

    let b = "fn func(x: f32) -> f32 {
	q = 10.;
} 

";

    assert_eq!(&do_parse(DEFINE.to_string()), b);
}

// swizzling is handled in the show_expr() function of wgsl.rs
// next to the matched pattern "syntax::Expr::Assignment(ref v2, ref op2, ref e2) =>"
#[test]
fn xyz() {
    const XYZ: &str = "

void main() {
  q.xw = vec2(-1, 3);

}";

    let b = "fn main()  {
	var qxw = q.xw;
	qxw = vec2<f32>(-1., 3.);
	q.x = qxw.x;
	q.w = qxw.y;
} 

";

    assert_eq!(&do_parse(XYZ.to_string()), b);
}

#[test]
fn minus_float() {
    const MINUS_FLOAT: &str = "
void main() {
  vec2 q = vec2(-1, 3);

}
";

    let b = "fn main()  {
	let q: vec2<f32> = vec2<f32>(-1., 3.);
} 

";

    assert_eq!(&do_parse(MINUS_FLOAT.to_string()), b);
}

#[test]
fn if_question_mark() {
    const IF_QUESTION: &str = "
void main() {
  float q = w?1:4;

}
";
    // TODO: this should be a var, not a let
    let b = "fn main()  {
	let q: f32; if (w) { q = 1; } else { q = 4; };
} 

";

    assert_eq!(&do_parse(IF_QUESTION.to_string()), b);
}

#[test]
fn for_convert_to_float() {
    const FOR_CONVERT_TO_FLOAT: &str = "
void main(  )
{    
    for(float i = 1; i < 2; i = i + 1)
    {

    }
}
";

    let b = "fn main()  {

	for (var i: f32 = 1.; i < 2.; i = i + 1.) {
	}

} 

";

    assert_eq!(&do_parse(FOR_CONVERT_TO_FLOAT.to_string()), b);
}

#[test]
fn define_func() {
    const DEFINES_FUNC: &str = "
#define texel(ax, p) texelFetch(ax(i), Bi(p(a)), ax(i))
#define q 12
#define t(pk, l) bobbyFisher(15, pk)
void main() {
   texel(ch0, f);
   texel(ch4, st);
   gold = q + t(GR, big);
 }
";

    let b = "fn main()  {
	textureLoad(BUFFER_ch0(i), vec2<i32>(Bi(f(a))));
	textureLoad(BUFFER_ch4(i), vec2<i32>(Bi(st(a))));
	gold = 12 + bobbyFisher(15, GR);
} 

";

    assert_eq!(&do_parse(DEFINES_FUNC.to_string()), b);
}

#[test]
fn range() {
    const RANGE: &str = "
#define range(i,a,b) for(int i = a; i <= b; i++)
void Simulation()
{
    range(i, -2, 2) range(j, -2, 2)
    {
        vec2 tpos = pos + vec2(i,j);
    }
}
";

    let b = "fn Simulation()  {

	for (var i: i32 = -2; i <= 2; i = i + 1) {
	for (var j: i32 = -2; j <= 2; j = j + 1) {
		let tpos: vec2<f32> = pos + vec2<f32>(i, j);
	}

	}

} 

";

    assert_eq!(&do_parse(RANGE.to_string()), b);
}

#[test]
fn inout2() {
    const INOUT: &str = "
void func(float a, inout vec4 P)
{
   P.x += 1.0;
   float a2 = P.w;
}

float func2( inout vec4 wert, vec3 c, inout float a)
{
 wert = 56;
 a = vec2(1,1);
 for (int i = 0; i < octaveNum; i++) {
    acc += vec2(noise(c), noise(p2 + vec3(0,0,10))) * amp;
    c = 23;
 }
}";

    let b = "fn func(a: f32, P: ptr<function, vec4<f32>>)  {
	(*P).x = (*P).x + (1.);
	let a2: f32 = (*P).w;
} 

fn func2(wert: ptr<function, vec4<f32>>, c: vec3<f32>, a: ptr<function, f32>) -> f32 {
	var c_var = c;
	(*wert) = 56;
	(*a) = vec2<f32>(1., 1.);

	for (var i: i32 = 0; i < octaveNum; i = i + 1) {
		acc = acc + (vec2<f32>(noise(c_var), noise(p2 + vec3<f32>(0., 0., 10.))) * amp);
		c_var = 23;
	}

} 

";

    assert_eq!(&do_parse(INOUT.to_string()), b);
}

#[test]
fn texel_fetch() {
    const TEXEL_FETCH: &str = "
void main() {
   vec4 wqe = texelFetch(ch0, q);
   vec4 wqe = texture(ch0, q / R);
   vec4 wqe = textureLod(ch0, q / R, 0.5);
 }
";

    let b = "fn main()  {
	var wqe: vec4<f32> = textureLoad(BUFFER_ch0, vec2<i32>(q));
	var wqe: vec4<f32> = sample_texture(BUFFER_ch0, q / R);
	let wqe: vec4<f32> = textureSampleLevel(BUFFER_ch0, buffer_sampler, q / R, f32(0.5));
} 

";

    assert_eq!(&do_parse(TEXEL_FETCH.to_string()), b);
}

#[test]
fn let_vs_varprivate() {
    const LET_VS_VARPRIVATE: &str = "
#define q 12
float q2;
void main() {
p = q;
q2 = 4;
  }
";

    let b = "var<private> q2: f32;
fn main()  {
	p = 12;
	q2 = 4;
} 

";

    assert_eq!(&do_parse(LET_VS_VARPRIVATE.to_string()), b);
}

#[test]
fn var_dot() {
    const VAR_DOT: &str = "
void main() {
  float atj = 4;
  atj.xy = vec2(5);
}
";

    let b = "fn main()  {
	var atj: f32 = 4.;
	var atjxy = atj.xy;
	atjxy = vec2<f32>(5.);
	atj.x = atjxy.x;
	atj.y = atjxy.y;
} 

";

    assert_eq!(&do_parse(VAR_DOT.to_string()), b);
}

#[test]
fn define_func_comma() {
    const DEFINE_FUNC_COMMA: &str = "
#define _sub   S(45);  
#define S(a) c+=char(a);  tp.x-=FONT_SPACE;

void main() {
    float c = 0.;
}

void f() {
  float aaa = bcbx, cxvb = 1;
}
";

    let b = "fn main()  {
	let c: f32 = 0.;
} 

fn f()  {
	let aaa: f32 = bcbx;
	let cxvb: f32 = 1.;
} 

";

    assert_eq!(&do_parse(DEFINE_FUNC_COMMA.to_string()), b);
}

#[test]
fn reassigned_arg() {
    const REASSIGNED_ARG: &str = "void func(vec2 fragColor, vec2 fragCoord)
{
    fragCoord = vec2(0.);
    fragColor = vec2(1.0);
}
";

    let b = "fn func(fragColor: vec2<f32>, fragCoord: vec2<f32>)  {
	var fragCoord_var = fragCoord;
	var fragColor_var = fragColor;
	fragCoord_var = vec2<f32>(0.);
	fragColor_var = vec2<f32>(1.);
} 

";

    assert_eq!(&do_parse(REASSIGNED_ARG.to_string()), b);
}

#[test]
fn const_let() {
    const CONST: &str = "
const float c = 1;
";

    let b = "let c: f32 = 1.;
";

    assert_eq!(&do_parse(CONST.to_string()), b);
}

#[test]
fn replace_mod_test() {
    const MOD: &str = "void main()  {
    float a = mod(g, q);
    float b = mod(mod(qr, to), mod(other, less));
} 
";

    let b = "fn main()  {
	let a: f32 = ((g) % (q));
	let b: f32 = ((((qr) % (to))) % (((other) % (less))));
} 

";

    assert_eq!(&do_parse(MOD.to_string()), b);
}

#[test]
fn clamp_convert() {
    const CLAMP: &str = "
void main() {
    vec2 x = clamp(v, 0, clamp(z, 0, 1));
    vec2 z;
    z = clamp(z, 0, 1);
}";

    let b = "fn main()  {
	let x: vec2<f32> = clamp(v, vec2<f32>(0.), clamp(z, vec2<f32>(0.), vec2<f32>(1.)));
	var z: vec2<f32>;
	z = clamp(z, 0, 1);
} 

";

    assert_eq!(&do_parse(CLAMP.to_string()), b);
}
