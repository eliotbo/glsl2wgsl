//! parses let keywords and checks if the declared variable
//! is reassigned later. If so, it replacees the "let" keyword
//! by "var", as per the syntax of WGSL.
//! This parser ignores scope. So if two variables have the same
//! name but different scopes, they will be considered as the same
//! variable.

use nom::Parser;

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until, take_while1};
use nom::character::complete::{alpha1, alphanumeric1, anychar, char, line_ending, multispace1};

use nom::combinator::{cut, eof, map, opt, peek, recognize, success, value, verify};
use nom::error::{ErrorKind, VerboseError, VerboseErrorKind};
use nom::multi::{fold_many0, many0, many_till};
use nom::sequence::{pair, preceded, terminated};

use nom::{Err as NomErr, IResult};

pub type ParserResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

// A constant parser that just forwards the value it’s parametered with without reading anything
// from the input. Especially useful as “fallback” in an alternative parser.
//
pub fn cnst<'a, T, E>(t: T) -> impl FnMut(&'a str) -> Result<(&'a str, T), E>
where
    T: 'a + Clone,
{
    move |i| Ok((i, t.clone()))
}

// End-of-input parser.
//
// Yields `()` if the parser is at the end of the input; an error otherwise.
pub fn eoi(i: &str) -> ParserResult<()> {
    if i.is_empty() {
        Ok((i, ()))
    } else {
        Err(NomErr::Error(VerboseError {
            errors: vec![(i, VerboseErrorKind::Nom(ErrorKind::Eof))],
        }))
    }
}

// A newline parser that accepts:
//
// - A newline.
// - The end of input.
pub fn eol(i: &str) -> ParserResult<()> {
    alt((
        eoi, // this one goes first because it’s very cheap
        value((), line_ending),
    ))(i)
}

// Apply the `f` parser until `g` succeeds. Both parsers consume the input.
pub fn till<'a, A, B, F, G>(mut f: F, mut g: G) -> impl FnMut(&'a str) -> ParserResult<'a, ()>
where
    F: FnMut(&'a str) -> ParserResult<'a, A>,
    G: FnMut(&'a str) -> ParserResult<'a, B>,
{
    move |mut i| loop {
        if let Ok((i2, _)) = g(i) {
            break Ok((i2, ()));
        }

        let (i2, _) = f(i)?;
        i = i2;
    }
}

// A version of many0 that discards the result of the parser, preventing allocating.
pub fn many0_<'a, A, F>(mut f: F) -> impl FnMut(&'a str) -> ParserResult<'a, ()>
where
    F: FnMut(&'a str) -> ParserResult<'a, A>,
{
    move |i| fold_many0(&mut f, || (), |_, _| ())(i)
}

/// Parse a string until the end of line.
///
/// This parser accepts the multiline annotation (\) to break the string on several lines.
///
/// Discard any leading newline.
pub fn str_till_eol(i: &str) -> ParserResult<&str> {
    map(
        recognize(till(alt((value((), tag("\\\n")), value((), anychar))), eol)),
        |i| {
            if i.as_bytes().last() == Some(&b'\n') {
                &i[0..i.len() - 1]
            } else {
                i
            }
        },
    )(i)
}

/// Parse a single comment.
pub fn comment(i: &str) -> ParserResult<&str> {
    preceded(
        char('/'),
        alt((
            preceded(char('/'), cut(str_till_eol)),
            preceded(char('*'), cut(terminated(take_until("*/"), tag("*/")))),
        )),
    )(i)
}

// Blank base parser.
//
// This parser succeeds with multispaces and multiline annotation.
//
// Taylor Swift loves it.
pub fn blank_space(i: &str) -> ParserResult<&str> {
    recognize(many0_(alt((multispace1, tag("\\\n")))))(i)
}

/// Parse several comments.
pub fn comments(i: &str) -> ParserResult<&str> {
    recognize(many0_(terminated(comment, blank_space)))(i)
}

#[inline]
fn identifier_pred(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

#[inline]
fn verify_identifier(s: &str) -> bool {
    !char::from(s.as_bytes()[0]).is_digit(10)
}

/// Parse an identifier (raw version).
fn identifier_str(i: &str) -> ParserResult<&str> {
    verify(take_while1(|x| identifier_pred(x)), verify_identifier)(i)
}

/// Parse a string that could be used as an identifier.
pub fn string(i: &str) -> ParserResult<String> {
    map(identifier_str, |x| String::from(x))(i)
}

pub fn read_type(i: &str) -> ParserResult<String> {
    map(tag(": ").and(till_space).and(tag(" = ")), |(x1, _x2)| {
        let mut colon = ": ".to_owned();
        colon.push_str(&x1.1);
        colon.push_str(" = ");
        colon
    })(i)
}

pub fn read_equal(i: &str) -> ParserResult<String> {
    map(tag(" = "), |x: &str| x.to_owned())(i)
}

pub fn either_type_or_not(i: &str) -> ParserResult<String> {
    map(alt((read_type, read_equal)), |x| x.to_owned())(i)
}

pub fn read_named_var(i: &str) -> ParserResult<(String, String)> {
    map(
        tag("let ").and(till_space_or_colon).and(either_type_or_not),
        |(x1, x2)| {
            let mut decl = "let ".to_owned();
            // decl.push_str(x1.0);
            decl.push_str(&x1.1);
            decl.push_str(&x2);
            (x1.1, decl)
        },
    )(i)
}

pub fn till_space_or_colon(i: &str) -> ParserResult<String> {
    map(
        many_till(anychar, peek(alt((tag(" "), tag(":"))))),
        |(parsed, _v)| parsed.iter().collect::<String>(),
    )(i)
}

pub fn till_space(i: &str) -> ParserResult<String> {
    map(many_till(anychar, peek(tag(" "))), |(parsed, v)| {
        let mut s = parsed.iter().collect::<String>();
        s.push_str(v);
        s
    })(i)
}

pub fn is_repeated<'a, 'b>(s: &'b str) -> impl FnMut(&'a str) -> ParserResult<bool> + 'b {
    move |i: &str| {
        map(opt(peek(many_till(anychar, peek(tag(s))))), |x| match x {
            Some(_) => true,
            None => false,
        })(i)
    }
}

pub fn get_named_var(i: &str) -> ParserResult<String> {
    map(
        tag("let ").and(till_space_or_colon).and(either_type_or_not),
        |(x1, _x2)| x1.1,
    )(i)
}

pub fn identifier(input: &str) -> ParserResult<&str> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        ))
        // make sure the next character is not part of the identifier
        .and(peek(verify(anychar, |x| !x.is_alphanumeric()))),
        |x: (&str, char)| {
            let ret = x.0;
            ret
        },
    )(input)
}

// // search (v, where v is an identifier) and replace by (num, which can be anychar)
// pub fn search_for_v2(i: &str, v: String) -> ParserResult<bool> {
//     // search for next instance of v
//     let x = opt(many_till(anychar, verify(identifier, |x| x == v)))(i)?;

//     if let (rest, Some((v_chars, _name))) = x {
//         let y = opt(many_till(anychar, tag(" = ")))(rest)?;
//         if let (_, Some(_)) = y {
//             return success(true)(rest);
//         } else {
//             return success(false)("");
//         }
//     } else {
//         return success(false)("");
//     }
// }

// pub fn full_identifier(i: &str) -> ParserResult<String> {
//     let (rest, iden) = preceded(
//         verify(count(anychar, 1), |cs: Vec<char>| {
//             let c = cs.iter().last().unwrap();
//             !c.is_alphanumeric() && !(c == &'_')
//         }),
//         identifier,
//     )(i)?;
//     // let ret = x.1.to_owned();
//     return Ok((rest, iden.to_owned()));
// }

const ALPHANUM_UNDER: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789";

// pub fn search_for_full_identifier(i: &str, v: String) -> ParserResult<bool> {
//     let x = opt(many_till(
//         anychar,
//         verify(preceded(is_not(ALPHANUM_UNDER), identifier), |x| x == v),
//     ))(i)?;

//     if let (rest, Some(_)) = x {
//         let (rest2, (rest_of_line, _)) = many_till(anychar, eol)(rest)?;
//         let rest_of_line = rest_of_line.iter().collect::<String>();

//         let y: ParserResult<Option<(Vec<char>, &str)>> =
//             opt(many_till(anychar, tag(" = ")))(rest_of_line.as_str());

//         if let Ok((_, Some(_))) = y {
//             return success(true)(rest2);
//         }
//     }

//     return success(false)(i);
// }

pub fn search_for_full_identifier<'a, 'b>(
    s: &'b str,
) -> impl FnMut(&'a str) -> ParserResult<bool> + 'b {
    move |i: &str| {
        let x = opt(many_till(
            anychar,
            verify(preceded(is_not(ALPHANUM_UNDER), identifier), |x: &str| {
                x == s
            }),
        ))(i)?;

        if let (rest, Some((_, _name))) = x {
            let (rest2, (rest_of_line, _)) = many_till(anychar, eol)(rest)?;
            let rest_of_line = rest_of_line.iter().collect::<String>();

            let y: ParserResult<Option<(Vec<char>, &str)>> =
                opt(many_till(anychar, tag(" = ")))(rest_of_line.as_str());

            if let Ok((_, Some(_))) = y {
                return success(true)(rest2);
            }
        }

        return success(false)(i);
    }
}

pub fn decl_is_reassigned(i: &str) -> ParserResult<bool> {
    let (rest, name) = map(get_named_var, |x| x)(i)?;

    let z: ParserResult<bool> = map(peek(search_for_full_identifier(&name)), |x| x)(rest);
    return z;
}

pub fn write_var_or_let(i: &str) -> ParserResult<String> {
    map(opt(peek(decl_is_reassigned)), |x| {
        if let Some(true) = x {
            "var ".to_owned()
        } else {
            "let ".to_owned()
        }
    })(i)
}

pub fn replace_1_let(i: &str) -> ParserResult<String> {
    map(
        many_till(anychar, write_var_or_let.and(tag("let "))),
        |(x1, (varlet, _))| {
            let mut v = x1.iter().collect::<String>();
            v.push_str(&varlet);
            v
        },
    )(i)
}

pub fn replace_all_let(i: &str) -> ParserResult<String> {
    map(many0(replace_1_let), |x2| x2.join(""))(i)
}

pub fn let2var_parser(i: &str) -> ParserResult<String> {
    map(
        replace_all_let.and(many_till(anychar, eof)),
        |(mut replaced_lets, (rest, _))| {
            let rest_of_script: String = rest.iter().collect();
            replaced_lets.push_str(&rest_of_script);
            replaced_lets
        },
    )(i)
}
