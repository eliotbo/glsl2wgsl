// This parser finds and replaces all instances of the GLSL texelFetch and texture functions
// by the WGLSL textureLoad function.

use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::combinator::{eof, map};
use nom::multi::{many0, many_till};
use nom::sequence::preceded;
use nom::Parser;

pub use crate::nom_helpers::*;

pub fn check_one_texel_fetch(i: &str) -> ParserResult<String> {
    let (rest, arguments) = map(
        many_till(
            anychar,
            preceded(tag("texelFetch"), function_call_args_anychar),
        ),
        |(before_texel_fetch, args)| {
            let mut ret = before_texel_fetch.iter().collect::<String>();

            ret = ret + "textureLoad(" + "BUFFER_" + args[0].as_str() + ", ";
            ret = ret + "vec2<i32>(" + args[1].as_str() + "))";

            ret
        },
    )(i)?;

    return Ok((rest, arguments));
}

pub fn check_one_texture(i: &str) -> ParserResult<String> {
    let (rest, arguments) = map(
        many_till(
            anychar,
            preceded(tag("texture"), function_call_args_anychar),
        ),
        |(before_texel_fetch, args)| {
            let mut ret = before_texel_fetch.iter().collect::<String>();

            ret = ret + "textureLoad(" + "BUFFER_" + args[0].as_str() + ", ";
            ret = ret
                + "vec2<i32>("
                + args[1].as_str()
                + "  /* 0 to 1 range -> CONVERT TO I32 */  ))";

            ret
        },
    )(i)?;

    return Ok((rest, arguments));
}

pub fn replace_all_texture_and_texel_fetch(i: &str) -> ParserResult<String> {
    let (_, replaced_texel_fetch) = map(
        many0(check_one_texel_fetch).and(many_till(anychar, eof)),
        |(s, (t, _q))| {
            let so_far = s.join("");
            let rest = t.iter().collect::<String>();

            so_far + &rest
        },
    )(i)?;

    let replaced_texel_fetch_and_texture = map(
        many0(check_one_texture).and(many_till(anychar, eof)),
        |(s, (t, _q))| {
            let so_far = s.join("");
            let rest = t.iter().collect::<String>();

            so_far + &rest
        },
    )(replaced_texel_fetch.as_str());

    if let Ok((_rest, r)) = replaced_texel_fetch_and_texture {
        return Ok(("", r));
    } else {
        return Ok(("", i.to_string()));
    }
}
