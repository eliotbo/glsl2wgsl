// This parser finds and replaces all instances of the GLSL texelFetch and texture functions
// by the WGLSL textureLoad function.

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, space0};
use nom::combinator::{eof, map, peek};
use nom::multi::{many0, many_till};
use nom::sequence::tuple;
use nom::Parser;

pub use crate::nom_helpers::*;

// pub fn check_one_texel_fetch(i: &str) -> ParserResult<String> {
//     let (rest, arguments) = map(
//         many_till(
//             anychar,
//             preceded(tag("texelFetch"), function_call_args_anychar),
//         ),
//         |(before_texel_fetch, args)| {
//             let mut ret = before_texel_fetch.iter().collect::<String>();

//             ret = ret + "textureLoad(" + "BUFFER_" + args[0].as_str() + ", ";
//             ret = ret + "vec2<i32>(" + args[1].as_str() + "))";

//             ret
//         },
//     )(i)?;

//     return Ok((rest, arguments));
// }

pub fn check_one_texture(i: &str) -> ParserResult<String> {
    let (rest, arguments) = map(
        many_till(
            anychar,
            tuple((
                alt((tag("\n"), tag("\t"), space0)),
                alt((tag("textureLod"), tag("texture"), tag("texelFetch"))),
                // alt((tag("z"), tag("kk"))),
                peek(tag("(")),
                function_call_args_anychar,
            )),
        ),
        |(before_texel_fetch, (space_before_texel_fetch, texture, _, args))| {
            let mut ret = before_texel_fetch.iter().collect::<String>() + space_before_texel_fetch;

            println!("args: {:?}", args);

            // fn textureSampleLevel(t: texture_2d<f32>,
            //                       s: sampler,
            //                       coords: vec2<f32>,
            //                       level: f32) -> vec4<f32>

            match texture {
                "texelFetch" => {
                    ret = ret + "textureLoad(" + "BUFFER_" + args[0].as_str() + ", ";
                    ret = ret + "vec2<i32>(" + args[1].as_str() + "))";
                }
                "textureLod" => {
                    println!("HTRR");
                    ret = ret
                        + "textureSampleLevel("
                        + "BUFFER_"
                        + args[0].as_str()
                        + ", buffer_sampler, ";
                    ret = ret + args[1].as_str() + ", ";
                    ret = ret + "f32(" + args[2].as_str() + "))";
                }
                "texture" => {
                    ret = ret + "sample_texture(" + "BUFFER_" + args[0].as_str() + ", ";
                    ret = ret + args[1].as_str() + ")";
                }

                _ => {}
            }

            ret
        },
    )(i)?;

    return Ok((rest, arguments));
}

pub fn replace_all_texture_and_texel_fetch(i: &str) -> ParserResult<String> {
    // let (_, replaced_texel_fetch) = map(
    //     many0(check_one_texel_fetch).and(many_till(anychar, eof)),
    //     |(s, (t, _q))| {
    //         let so_far = s.join("");
    //         let rest = t.iter().collect::<String>();

    //         so_far + &rest
    //     },
    // )(i)?;

    // let replaced_texel_fetch_and_texture = map(
    //     many0(check_one_texture).and(many_till(anychar, eof)),
    //     |(s, (t, _q))| {
    //         let so_far = s.join("");
    //         let rest = t.iter().collect::<String>();

    //         so_far + &rest
    //     },
    // )(replaced_texel_fetch.as_str());

    // if let Ok((_rest, r)) = replaced_texel_fetch_and_texture {
    //     return Ok(("", r));
    // } else {
    //     return Ok(("", i.to_string()));
    // }

    map(
        many0(check_one_texture).and(many_till(anychar, eof)),
        |(s, (t, _q))| {
            let so_far = s.join("");
            let rest = t.iter().collect::<String>();

            so_far + &rest
        },
    )(i)
}
