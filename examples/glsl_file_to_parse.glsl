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
  return f.x * (f.y+0.3)*(f.z+0.3)*2.0; 
}

void SetTextPosition(float x, float y)  
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

//--- display float value ---
float drawFloat(float value, int prec, int maxDigits)
{ 
  float tpx = tp.x-FONT_SPACE*float(maxDigits);
  float c = 0.;
  if (value < 0.0) 
  { 
    c = char(45); // write minus sign
    value = -value;
  }
  tp.x -= FONT_SPACE;
  c += drawInt(int(value),1);
  c += char(46); BLANK; // add dot 
  c += drawFract(fract(value), prec);
  tp.x = min(tp.x, tpx);
  return c;
}

float drawFloat(float value)           {return drawFloat(value,2,5);} 

float drawFloat(float value, int prec) {return drawFloat(value,prec,2);} 

float drawInt(int value)               {return drawInt(value,1);}

//== geometric drawings ===========================================

//--- draw line segment from A to B ---
float drawLineSegment(vec2 A, vec2 B, float r)
{
    vec2 g = B - A;
    vec2 h = uv - A;
    float d = length(h - g * clamp(dot(g, h) / dot(g,g), 0.0, 1.0));
	return smoothstep(r, 0.5*r, d);
}
//--- draw circle at pos with given radius ---
float circle(in vec2 pos, in float radius, in float halo)
{
  return clamp (halo * (radius - length(uv-pos)), 0.0, 1.0);
}

//=================================================================

const vec3 headColor = vec3(0.90, 0.60, 0.20);
const vec3 backColor = vec3(0.15, 0.10, 0.10);
const vec3 mpColor   = vec3(0.99, 0.99, 0.00);
const vec3 mxColor   = vec3(1.00, 0.00, 0.00);
const vec3 myColor   = vec3(0.00, 1.00, 0.00);
      vec3 dotColor  = vec3(0.50, 0.50, 0.00);
      vec3 drawColor = vec3(1.0, 1.0, 0.0);
      vec3 vColor    = backColor;  // value color

float aspect = 1.0;
vec2 pixelPos   = vec2(0.0);  // pixel position:  0 .. resolution-1
vec2 mousePos   = vec2(200);  // mouse pixel position  
vec2 lp         = vec2(0.5);  // last mouse position 
vec2 mp         = vec2(0.5);  // current mouse position 
vec2 resolution = vec2(0.0);  // window resolution

//----------------------------------------------------------------
void SetColor(float red, float green, float blue)
{
  drawColor = vec3(red,green,blue);
}
//----------------------------------------------------------------
void WriteFloat(const in float fValue 
               ,const in int maxDigits 
               ,const in int decimalPlaces)
{
  vColor = mix(vColor, drawColor, drawFloat (fValue, decimalPlaces));
  BLANK;
}
//----------------------------------------------------------------
void WriteInteger(const in int iValue)
{
  vColor = mix(vColor, drawColor, drawInt (iValue));
  BLANK;
}
//----------------------------------------------------------------
void WriteDate()
{
  float c = 0.0;
  c += drawInt(int(iDate.x));       _sub;
  c += drawInt(int(iDate.y +1.0));  _sub;
  c += drawInt(int(iDate.z)); _
  vColor = mix(vColor, drawColor, c);
}
//----------------------------------------------------------------
void WriteTime()
{
  float c = 0.0;
  c += drawInt(int(mod(iDate.w / 3600.0, 24.0)));    _ddot;
  c += drawInt(int(mod(iDate.w / 60.0 ,  60.0)),2);  _ddot;
  c += drawInt(int(mod(iDate.w,          60.0)),2);  _
  vColor = mix(vColor, drawColor, c);
}
//----------------------------------------------------------------
void WriteFPS()
{
  // print Frames Per Second - FPS  see https://www.shadertoy.com/view/lsKGWV
  //float fps = (1.0 / iTimeDelta + 0.5);
  float fps = iFrameRate;
  SetColor (0.8, 0.6, 0.3);
  WriteFloat(fps, 6, 1);
  float c = 0.0;
  _f _p _s
  // _ WriteFloat(iTimeDelta*1000., 8, 1); _m _s
  vColor = mix(vColor, drawColor, c);
}
//----------------------------------------------------------------
void WriteMousePos(float ytext, vec2 mPos)
{
  int digits = 3;
  float radius = resolution.x / 200.;

  // print dot at mPos.xy
  if (iMouse.z > 0.0) dotColor = mpColor;
  float r = length(abs(mPos.xy) - pixelPos) - radius;
  vColor += mix(vec3(0), dotColor, (1.0 - clamp(r, 0.0, 1.0)));

  // print first mouse value
  SetTextPosition(1., ytext);

  // print mouse position
  if (ytext == 7.)
  {
    drawColor = mxColor;
    WriteFloat(mPos.x,6,3);
    BLANK;
    drawColor = myColor;
    WriteFloat(mPos.y,6,3);
  }
  else
  {
    drawColor = mxColor;
    WriteInteger(int(mPos.x));
    BLANK;
    drawColor = myColor;
    WriteInteger(int(mPos.y));
  }
}    
//----------------------------------------------------------------
void WriteText1()
{
  SetTextPosition(1.,1.);
  float c = 0.0;
  _star _ _V _i _e _w _ _S _h _a _d _e _r 
  
  _ _D _a _t _a _ _2 _ _ _v _1 _dot _5 _ _star 
      
  vColor += c * headColor;
}
//----------------------------------------------------------------
void WriteWebGL()     // browser test: http://webglreport.com/?v=2
{
  SetTextPosition(1.,3.);
  float c = 0.0;
  _W _e _b _G _L
  

     _2


      
  vColor += c * headColor;
}
//----------------------------------------------------------------
void WriteTestValues()
{
  float c = 0.0;
  SetTextPosition(1.,12.);
    c += drawInt(123, 8);
  _ c += drawInt(-1234567890);    // right now !!!
  _ c += drawInt(0);
  _ c += drawInt(-1);
  _ c += drawFloat(-123.456, 3);     // right now !!!

  SetTextPosition(1.,13.);
    c += drawInt(-123, 8);
  _ c += drawInt(1234567890,11);
    c += drawFloat(0.0,0,0);
  _ c += drawFloat(1.0,0,0);
  _ c += drawFloat(654.321, 3);      // nearly right
  _ c += drawFloat(999.9, 1);
  _ c += drawFloat(pow(10., 3.),1);
    c += drawFloat(pow(10., 6.),1);
  
  SetTextPosition(1.,14.);
  c += drawFloat(exp2(-126.0),60);
  vColor += c * headColor;
}
//---------------------------------------------------------
// draw ring at given position
//---------------------------------------------------------
float ring(vec2 pos, float radius, float thick)
{
  return mix(1.0, 0.0, smoothstep(thick, thick + 0.01, abs(length(uv-pos) - radius)));
}
//----------------------------------------------------------------
// define center coodinates
//#define CC(c) ((2.0 * c / resolution.xy - 1.0) * ratio)
#define CC(c) ((2.0*c.xy-iResolution.xy) / iResolution.y)

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
  resolution = iResolution.xy;
  aspect = resolution.x / resolution.y;    // aspect ratio
  vec2 ratio = vec2(aspect, 1.0);
  pixelPos = fragCoord.xy;  //  0 .. resolution
  mousePos = iMouse.xy;     //  0 .. resolution
  uv = CC(fragCoord.xy);    // -1 .. 1 pixel position
  mp = CC(abs(iMouse.xy));  // -1 .. 1 current mouse position
  lp = CC(abs(iMouse.zw));  // -1 .. 1 last mouse position
  
  // draw screen axis
  vColor = mix(vColor, vec3(0.2), drawLineSegment(vec2(-99.,0.), vec2(99.,0.), 0.01));
  vColor = mix(vColor, vec3(0.2), drawLineSegment(vec2(0.,-99.), vec2(0.,99.), 0.01));

  // version & test values   
  WriteText1();
  WriteWebGL();  
  WriteTestValues();

  // mouse position & coordinates
  WriteMousePos(5., iMouse.zw);  // last position
  WriteMousePos(6., iMouse.xy);  // current position

  // write r = circle Radius
  float radius = length(mp - lp);
  SetColor (0.9, 0.9, 0.2);
  float c = 0.0;
  _  _r _eq
  vColor += c * drawColor;
  WriteFloat (radius,6,2);

  // draw circle
  if (iMouse.z > 0.0)
  {
    float intensity = ring(lp, radius, 0.01);
    drawColor = vec3(1.5, 0.4, 0.5);
    vColor = mix(vColor, drawColor, intensity*0.2);
  }

  // Resolution
  SetTextPosition(27.0, 1.0);
  SetColor (0.8, 0.8, 0.8);
  WriteInteger(int(iResolution.x));  _star _  vColor += c * drawColor;
  WriteInteger(int(iResolution.y));

  // KeyPressed
  SetTextPosition(1.0, 16.);
  SetColor (0.9, 0.7, 0.8);
  for (int ci=0; ci<256; ci++)
  if (keyClick(ci)) WriteInteger(ci);
    
  // Date
  SetTextPosition(1.0, 19.);
  SetColor (0.9, 0.9, 0.4);
  WriteDate();
  BLANK

  // Time
  SetColor (1.0, 0.0, 1.0);
  WriteTime();
  BLANK

  // Frame Counter
  SetColor (0.4, 0.7, 0.4);
  WriteInteger(iFrame);
  BLANK

  // Shader Time
  SetColor (0.0, 1.0, 1.0);
  WriteFloat(iTime, 6, 2);
  BLANK

  // Frames Per Second
  WriteFPS();

  fragColor = vec4(vColor,1.0);
}


