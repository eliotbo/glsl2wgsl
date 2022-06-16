use crate::do_parse;

#[test]
fn single_line_if() {
    let a: &str = // ... 
"void norm(vec3 po) {
  if (r.x > d.x)  { 
    r = d;
    t =3;
  }
}";

    let b = // ...
"norm(po: vec3<f32>)  {
	if (r.x > d.x) {
		r = d;
		t = 3;
	}
} 

";

    assert_eq!(&do_parse(a.to_string()), b);
}

#[test]
fn two_lines_if() {
    let a: &str = // ... 
"void norm(vec3 po) {
  if (r.x > d.x)  { 
    r = d;
    t =3;
  }
} ";

    let b = // ...
"norm(po: vec3<f32>)  {
	if (r.x > d.x) {
		r = d;
		t = 3;
	}
} \n\n";

    assert_eq!(&do_parse(a.to_string()), b);
}

#[test]
fn simple_vec2() {
    let a: &str = // ... 
"vec2 e = vec2(3.0);
float b = 1.0;";

    let b = // ...
"let e: vec2<f32> = vec2<f32>(3.);
let b: f32 = 1.;
";

    assert_eq!(&do_parse(a.to_string()), b);
}

#[test]
fn simple_const() {
    let a: &str = // ... 
"const vec2 e = vec2(.00035, -.00035);";

    let b = // ...
"const e: vec2<f32> = vec2<f32>(0.00035, -0.00035);
";

    assert_eq!(&do_parse(a.to_string()), b);
}

#[test]
fn func_proto() {
    const FUNC_PROTO: &str = "
vec3 norm(vec3 po) {}
vec2 norm2(vec2 wq) {}
";

    let b = // ...
"norm(po: vec3<f32>) -> vec3<f32> {
} 

norm2(wq: vec2<f32>) -> vec2<f32> {
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

    let b = // ...
"norm(po: vec3<f32>) -> vec3<f32> {
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

    let b = // ...
"var rd: vec4<f32> = vec4<f32>(0.);
norm(po: vec3<f32>)  {
	rd.x = rd.x * (2. + 3.);
} 

";

    assert_eq!(&do_parse(ASSIGN_OP.to_string()), b);
}

#[test]
fn for_loop() {
    const FOR_LOOP: &str = "void main() { for(int i = 0; i < 120; i++) { a = 3; } }";
    let b = // ...
"main()  {

	for (var i: i32 = 0; i < 120; i = i + 1) {
		a = 3;
	}

} 

";

    assert_eq!(&do_parse(FOR_LOOP.to_string()), b);
}

#[test]
fn array_decl() {
    const ARRAYED_DECLARATION: &str = "void norm(vec3 po) {
  float r = 2.0, e = 1.0;
}";

    let b = // ...
"norm(po: vec3<f32>)  {
	let r: f32 = 2.;
	let e: f32 = 1.;
} 

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

    let b = // ...
"norm(po: vec3<f32>)  {
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

#[test]
fn inout() {
    const IN_OUT: &str = "void func( out vec4 fragColor, in vec2 fragCoord )
{}";

    let b = // ...
"func(fragColor: vec4<f32>, fragCoord: vec2<f32>)  {
} 

";

    assert_eq!(&do_parse(IN_OUT.to_string()), b);
}

#[test]
fn simple_struct() {
    const SIMPLE_STRUCT: &str = "struct Light
{
float intensity;
};
";
    let b = // ...
"struct Light {
	intensity: f32;
};
";

    assert_eq!(&do_parse(SIMPLE_STRUCT.to_string()), b);
}

#[test]
fn array() {
    const ARRAY: &str = "const float yaa[2] = float[2](5.5, 8.7);";

    let b = // ...
"const yaa: array<f32,2> = array<f32,2>(5.5, 8.7);
";

    assert_eq!(&do_parse(ARRAY.to_string()), b);
}

#[test]
fn two_fn() {
    const TWO_FN: &str = "
void main() {}
void main() {}
";

    let b = // ...
"main()  {
} 

main()  {
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

    let b = // ...
"main()  {
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
fn define() {
    const DEFINE: &str = "
#define texel(a, p) texelFetch(a, Bi(p), 0)
#define blah 3.4

float hein(float x) {
  q = texel(tt, bb);
}
";

    let b = // ...
"var<private> blah = 3.4;

hein(x: f32) -> f32 {
	q = textureLoad(BUFFER_tt, vec2<i32>(Bi(bb)));
} 

";

    assert_eq!(&do_parse(DEFINE.to_string()), b);
}

#[test]
fn xyz() {
    const XYZ: &str = "

void main() {
  q.xy = vec2(-1, 3);

  blah.rg += 2;
}";
    let b = // ...
"main()  {
	var qxy = q.xy;
	qxy = vec2<f32>(-1., 3.);
	q.x = qxy.x;
	q.y = qxy.y;
	var blahrg = blah.rg;
	blahrg = blah.rg + (2);
	blah.r = blahrg.r;
	blah.g = blahrg.g;
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

    let b = // ...
"main()  {
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
    let b = // ...
"main()  {
	let q: f32; if (w) { q = 1; } else { q = 4; };
} 

";

    assert_eq!(&do_parse(IF_QUESTION.to_string()), b);
}

#[test]
fn for_convert_to_float() {
    const FOR_CONVERT_TO_FLOAT: &str = "
void mainImage(  )
{    
    for(float i = 1; i < 2; i = i + 1)
    {

    }
}
";

    let b = // ...
"mainImage()  {

	for (var i: f32 = 1.; i < 2.; i = i + 1.) {
	}

} 

";

    assert_eq!(&do_parse(FOR_CONVERT_TO_FLOAT.to_string()), b);
}

// TODO: out keywords means the wgsl version should return the corresponding value
#[test]
fn out_in() {
    const OUT: &str = "
void func( out vec4 U, in vec2 pos )
{ a= 5;}";

    let b = // ...
"func(U: vec4<f32>, pos: vec2<f32>)  {
	a = 5;
} 

";

    assert_eq!(&do_parse(OUT.to_string()), b);
}

#[test]
fn define_func() {
    const DEFINES_FUNC: &str = "
#define texel(ax, p) elFechax(ax, Bi(p), ax)
#define q 12
#define t(pk, l) bobbyFisher(15, pk)
void main() {
   texel(ch0, q);
   texel(ch4, st);
   gold = t(GR, big);
 }
";

    let b = // ...
"var<private> q = 12;

main()  {
	elFechax(ch0, Bi(q), ch0);
	elFechax(ch4, Bi(st), ch4);
	gold = t(GR, big);
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

    let b = // ...
"Simulation()  {

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

float func2(float c, inout vec4 wert, inout float a)
{
 wert = 56;
 a = vec2(1,1);
 c = 23;
}";

    let b = // ...
"func(a: f32, P: ptr<function, vec4<f32>>)  {
	(*P).x = (*P).x + (1.);
	let a2: f32 = (*P).w;
} 

func2(c: f32, wert: ptr<function, vec4<f32>>, a: ptr<function, f32>) -> f32 {
	(*wert) = 56;
	(*a) = vec2<f32>(1., 1.);
	c = 23;
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
 }
";

    let b = // ...
"main()  {
	var wqe: vec4<f32> = textureLoad(BUFFER_ch0, vec2<i32>(q));
	let wqe: vec4<f32> = textureLoad(BUFFER_ch0, vec2<i32>(q / R  /* 0 to 1 range -> CONVERT TO I32 */  ));
} 

";

    assert_eq!(&do_parse(TEXEL_FETCH.to_string()), b);
}

#[test]
fn let_vs_varprivate() {
    const LET_VS_VARPRIVATE: &str = "
float q;
#define q 12
float q = 1;
void main() {

// q = 4;
  }
";

    let b = // ...
"var<private> q: f32;
var<private> q = 12;

let q: f32 = 1.;
main()  {
} 

";

    assert_eq!(&do_parse(LET_VS_VARPRIVATE.to_string()), b);
}

#[test]
fn var_dot() {
    const VAR_DOT: &str = "
 float t = 1;
void main() {
  float atj = 4;
  atj.xy = 5;
}
";

    let b = // ...
"let t: f32 = 1.;
main()  {
	let atj: f32 = 4.;
	var atjxy = atj.xy;
	atjxy = 5;
	atj.x = atjxy.x;
	atj.y = atjxy.y;
} 

";

    assert_eq!(&do_parse(VAR_DOT.to_string()), b);
}