// use glsl2wgsl::parser::Parse;
use glsl2wgsl::nom_helpers::Span;
use glsl2wgsl::parser::Parse;
use glsl2wgsl::syntax;
// use glsl::*;
use glsl2wgsl::let2var::let2var_parser;
use glsl2wgsl::parse_func_defines::func_definition_parser;
use glsl2wgsl::replace_defines::*;
use glsl2wgsl::replace_inouts::{replace_inouts, search_and_replace_void};
use glsl2wgsl::replace_main::replace_main_line;
use glsl2wgsl::replace_texel_fetch::*;
use glsl2wgsl::replace_unis::uniform_vars_parser;
use glsl2wgsl::transpiler::wgsl::show_translation_unit;

use std::fs;

#[allow(dead_code)]
const ALL: &str = "

//=================================================================
// ViewShaderData2.glsl    
//   v1.0  2017-04-11  initial release
//                     !!! BUGS: integer and float conversion displays 
//                               wrong values in some cases !!! 
//   v1.1  2017-04-12  char() corrections
//                     convertion routines corrected by Timo Kinnunen!  
//   v1.2  2018-01-07  display WebGL version
//   v1.3  2019-05-25  key input added
//   v1.4  2019-05-25  corrections from FabriceNeyret2
//   v1.5  2020-12-28  correction because of iMouse data changes!
//         2020-12-28  working on...
//
// Display shader data:  
//   date, time, frameCount, runtime, fps, resolution & mouse positions.
// Click and Drag mouse button to display last & current mouse position.
// Press keys to show current pressed key values.
// Use Ctrl-mousewheel to change resolution.
// Press Alt-CursorDown to reset time & Alt-CursorUp to toggle pause.
//
// This release 2 uses the font texture to display integer and float values.
// useful shader infos:
//         font:  https://www.shadertoy.com/view/MtVXRd
//      numbers:  https://www.shadertoy.com/view/llySRh
//    version 1:  https://www.shadertoy.com/view/llcXDn
//    KeyTester:  https://www.shadertoy.com/view/llVSRm
// KeyTester v2:  https://www.shadertoy.com/view/XsycWw
// shaderValues:  https://www.shadertoy.com/view/llySRh
//   nice watch:  https://www.shadertoy.com/view/lsXGz8
// https://shadertoyunofficial.wordpress.com/2016/07/20/special-shadertoy-features/
//=================================================================

//== key handling ===

#define keyToggle(ascii) (texelFetch(iChannel3,ivec2(ascii,2),0).x > 0.)
#define keyDown(ascii)   (texelFetch(iChannel3,ivec2(ascii,1),0).x > 0.)
#define keyClick(ascii)  (texelFetch(iChannel3,ivec2(ascii,0),0).x > 0.)

//== font handling ==

#define FONT_BUFFER iChannel2
#define FONT_SPACE 0.5

vec2 uv = vec2(0.0);  // -1 .. 1

vec2 tp = vec2(0.0);  // text position

//--- access font image of ascii code characters ---

#define BLANK tp.x-=FONT_SPACE;
#define _     tp.x-=FONT_SPACE;

#define S(a) c+=char(a);  tp.x-=FONT_SPACE;

#define _note  S(10);   //
#define _star  S(28);   // *
#define _smily S(29);   // :-)        
#define _exc   S(33);   // !
#define _add   S(43);   // +
#define _comma S(44);   // ,
#define _sub   S(45);   // -
#define _dot   S(46);   // .
#define _slash S(47);   // /

#define _0 S(48);
#define _1 S(49);
#define _2 S(50);
#define _3 S(51);
#define _4 S(52);
#define _5 S(53);
#define _6 S(54);
#define _7 S(55);
#define _8 S(56);
#define _9 S(57);
#define _ddot S(58);   // :
#define _sc   S(59);   // ;
#define _less S(60);   // <
#define _eq   S(61);   // =
#define _gr   S(62);   // >
#define _qm   S(63);   // ?
#define _at   S(64);   // at sign

#define _A S(65);
#define _B S(66);
#define _C S(67);
#define _D S(68);
#define _E S(69);
#define _F S(70);
#define _G S(71);
#define _H S(72);
#define _I S(73);
#define _J S(74);
#define _K S(75);
#define _L S(76);
#define _M S(77);
#define _N S(78);
#define _O S(79);
#define _P S(80);
#define _Q S(81);
#define _R S(82);
#define _S S(83);
#define _T S(84);
#define _U S(85);
#define _V S(86);
#define _W S(87);
#define _X S(88);
#define _Y S(89);
#define _Z S(90);

#define _a S(97);
#define _b S(98);
#define _c S(99);
#define _d S(100);
#define _e S(101);
#define _f S(102);
#define _g S(103);
#define _h S(104);
#define _i S(105);
#define _j S(106);
#define _k S(107);
#define _l S(108);
#define _m S(109);
#define _n S(110);
#define _o S(111);
#define _p S(112);
#define _q S(113);
#define _r S(114);
#define _s S(115);
#define _t S(116);
#define _u S(117);
#define _v S(118);
#define _w S(119);
#define _x S(120);
#define _y S(121);
#define _z S(122);

//---------------------------------------------------------
// return font image intensity of character ch at text position tp
//---------------------------------------------------------

float char(int ch)    // old versions
{ vec4 f = texture(FONT_BUFFER,clamp(tp,0.,1.)/16.+fract(floor(vec2(ch,15.999-float(ch)/16.))/16.));
  return f.x * (f.y+0.3)*(f.z+0.3)*2.0;   // 3d
}
//  vec4 f = texture(FONT_BUFFER,clamp(tp,0.,1.)/16.+fract(floor(vec2(ch,16.-(1e-6)-floor(ch)/16.))/16.));  

/*
float char(int ch)    // new version
{
  vec4 f = any(lessThan(vec4(tp,1,1), vec4(0,0,tp))) 
               ? vec4(0) 
               : texture(FONT_BUFFER,0.0625*(tp + vec2(ch - ch/16*16,15 - ch/16)));  
//  if (iMouse.z > 0.0) return f.x; else   // 2d
  return f.x * (f.y+0.3)*(f.z+0.3)*2.0;   // 3d
}
*/

//----------------------------------------------------------------
// set text starting position to x=line, y=column (left/top = 1,1) 
//----------------------------------------------------------------
void SetTextPosition(float x, float y)  //
{
  tp = 10.0*uv;
  tp.x = tp.x +17. - x;
  tp.y = tp.y -9.4 + y;
}

//== value drawings =======================================

//--- display number fraction with leading zeros --- 
float drawFract(float value, int digits)
{ 
  float c = 0.0;
  value = fract(value) * 10.0;
  for (int ni = 1; ni < 60; ni++) 
  {
    c += char(48 + int(value)); // add 0..9
    tp.x -= FONT_SPACE;
    digits -= 1;
    value = fract(value) * 10.0;
    if (digits <= 0 || value == 0.0) break;
  } 
  tp.x -= FONT_SPACE*float(digits);
  return c;
}
                                                                                                             
//--- display integer value --- 
int maxInt(int a, int b) { return a>b?a:b;}  // to run on iPad

//--- display integer value --- 
float drawInt(int value, int minDigits)
{
  float c = 0.;
  if (value < 0) 
  { value = -value;
    if (minDigits < 1) minDigits = 1;
    else minDigits--;
    _sub                   // add minus char
  } 
  int fn = value, digits = 1; // get number of digits 
  for (int ni=0; ni<10; ni++)
  {
    fn /= 10;
    if (fn == 0) break;
    digits++;
  } 
  digits = maxInt(minDigits, digits);   // WebGL
//digits = max(minDigits, digits);      // WebGL2
  tp.x -= FONT_SPACE * float(digits);
  for (int ni=1; ni < 11; ni++) 
  { 
    tp.x += FONT_SPACE; // space
    c += char(48 + value%10);
    value /= 10; // add 0..9 
    if (ni >= digits) break;
  } 
  tp.x -= FONT_SPACE * float(digits);
  return c;
}


";

// TODO: fix the newline for statements
// 1: in and out keywords in function arguments
// THEN: big clean

fn main() {
    //
    let replaced_defines_func = func_definition_parser(&ALL).unwrap().1;

    // definition_parser(..) must be placed after func_definition_parser(..), because
    // the former erases all lines starting by #define
    let replaced_defines = definition_parser(&replaced_defines_func).unwrap().1;

    println!("replaced_defines: {:?}", replaced_defines);

    let trans = syntax::TranslationUnit::parse(Span::new(&replaced_defines)).unwrap();

    let mut buf: String = String::new();
    show_translation_unit(&mut buf, &trans);

    buf = let2var_parser(&buf).unwrap().1;
    buf = uniform_vars_parser(&buf).unwrap().1;

    buf = replace_main_line(&buf).unwrap().1;
    buf = replace_inouts(&buf).unwrap().1;
    buf = search_and_replace_void(&buf).unwrap().1;
    buf = replace_all_texture_and_texel_fetch(&buf).unwrap().1;

    fs::write("./foo.txt", &replaced_defines).expect("Unable to write file");

    // println!("{:?}", trans);
    println!("{:?}", buf);
}
