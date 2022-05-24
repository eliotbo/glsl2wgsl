use glsl2wgsl::parser::Parse;
use glsl2wgsl::syntax;
// use glsl::*;
use glsl2wgsl::do_parse;
use glsl2wgsl::let2var::*;
use glsl2wgsl::transpiler::wgsl::show_translation_unit;

use glsl2wgsl::parsers::nom_helpers::{blank_space2, Span};

use nom::combinator::peek;

use std::fs;

const TEST1: &str = "
float yu a = 1;
float sa = 1;
";

// const TEST1: &str = "

// mat3 getRotZMat(float a){return mat3(cos(a),-sin(a),0.,sin(a),cos(a),0.,0.,0.,1.);}

// float dstepf = 0.0;

// float map(vec3 p)
// {
// 	p.x += sin(p.z*1.8);
//     p.y += cos(p.z*.2) * sin(p.x*.8);
// 	p *= getRotZMat(p.z*0.8+sin(p.x)+cos(p.y));
//     p.xy = mod(p.xy, 0.3) - 0.15;
// 	dstepf += 0.003;
// 	return length(p.xy);
// }

// void mainImage( out vec4 fragColor, in vec2 fragCoord )
// {
// 	vec2 uv = (fragCoord - iResolution.xy*.5 )/iResolution.y;
//     vec3 rd = normalize(vec3(uv, (1.-dot(uv, uv)*.5)*.5));
//     vec3 ro = vec3(0, 0, iTime*1.26), col = vec3(0), sp;
// 	float cs = cos( iTime*0.375 ), si = sin( iTime*0.375 );
//     rd.xz = mat2(cs, si,-si, cs)*rd.xz;
// 	float t=0.06, layers=0., d=0., aD;
//     float thD = 0.02;
// 	for(float i=0.; i<250.; i++)
// 	{
//         if(layers>15. || col.x > 1. || t>5.6) break;
//         sp = ro + rd*t;
//         d = map(sp);
//         aD = (thD-abs(d)*15./16.)/thD;
//         if(aD>0.)
// 		{
//             col += aD*aD*(3.-2.*aD)/(1. + t*t*0.25)*.2;
//             layers++;
// 		}
//         t += max(d*.7, thD*1.5) * dstepf;
// 	}
//     col = max(col, 0.);
//     col = mix(col, vec3(min(col.x*1.5, 1.), pow(col.x, 2.5), pow(col.x, 12.)),
//               dot(sin(rd.yzx*8. + sin(rd.zxy*8.)), vec3(.1666))+0.4);
//     col = mix(col, vec3(col.x*col.x*.85, col.x, col.x*col.x*0.3),
//              dot(sin(rd.yzx*4. + sin(rd.zxy*4.)), vec3(.1666))+0.25);
// 	fragColor = vec4( clamp(col, 0., 1.), 1.0 );
// }
// ";

fn main() {
    // println!("{}", do_parse(TEST1.to_owned()));
    println!("{:?}", blank_space2(Span::new(TEST1)));

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
