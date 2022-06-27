// parses let keywords and checks if the declared variable
// is reassigned later. If so, it replacees the "let" keyword
// by "var", as per the syntax of WGSL.
// This parser ignores scope. So if two variables have the same
// name but different scopes, they will be considered as the same
// variable.

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_while1};
use nom::character::complete::{alpha1, alphanumeric1, anychar};
use nom::combinator::{eof, map, opt, peek, recognize, success, verify};
use nom::multi::{many0, many_till};
use nom::sequence::{pair, preceded};
use nom::Parser;

use crate::nom_helpers::*;

#[inline]
fn verify_identifier2(s: &str) -> bool {
    !char::from(s.as_bytes()[0]).is_digit(10)
}

/// Parse an identifier (raw version).
fn identifier_str(i: &str) -> ParserResult<&str> {
    verify(take_while1(|x| identifier_pred(x)), verify_identifier2)(i)
}

/// Parse a string that could be used as an identifier.
pub fn string(i: &str) -> ParserResult<String> {
    map(identifier_str, |x| String::from(x))(i)
}

// pub reassignment(i: &str) -> ParserResult<String> {
//     alt((tag(" = "), ))
// }

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

pub fn till_space(i: &str) -> ParserResult<String> {
    map(many_till(anychar, peek(tag(" "))), |(parsed, v)| {
        let mut s = parsed.iter().collect::<String>();
        s.push_str(v);
        s
    })(i)
}

// pub fn is_repeated<'a, 'b>(s: &'b str) -> impl FnMut(&'a str) -> ParserResult<bool> + 'b {
//     move |i: &str| {
//         map(opt(peek(many_till(anychar, peek(tag(s))))), |x| match x {
//             Some(_) => true,
//             None => false,
//         })(i)
//     }
// }

pub fn till_space_or_colon(i: &str) -> ParserResult<String> {
    map(
        many_till(anychar, peek(alt((tag(" "), tag(":"))))),
        |(parsed, _v)| parsed.iter().collect::<String>(),
    )(i)
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

// pub fn search_identifier_assignment<'a, 'b>(
//     s: &'b str,
// ) -> impl FnMut(&'a str) -> ParserResult<bool> + 'b {
//     move |i: &str| {
//         let (rest, x) = opt(many_till(
//             anychar,
//             verify(preceded(is_not(ALPHANUM_UNDER), identifier), |x: &str| {
//                 x == s
//             }),
//         ))(i)?;

//         println!("rest: {:?}", rest);
//         println!("x: {:?}", x);
//         if let Some((_, _name)) = x {
//             let (rest2, (rest_of_line, _)) = many_till(anychar, eol)(rest)?;
//             let rest_of_line = rest_of_line.iter().collect::<String>();

//             let y: ParserResult<Option<(Vec<char>, &str)>> =
//                 opt(many_till(anychar, tag(" = ")))(rest_of_line.as_str());

//             if let Ok((_, Some(_))) = y {
//                 println!("succ y: {:?} \n {:?}", y, rest2);
//                 return success(true)(rest2);
//             }
//         }

//         println!("FALSE x: {:?}", x);

//         return success(false)(rest);
//     }
// }

pub fn search_identifier_assignment<'a, 'b>(
    s: &'b str,
) -> impl FnMut(&'a str) -> ParserResult<bool> + 'b {
    move |i: &str| {
        let (rest, _) = many_till(
            anychar,
            verify(preceded(is_not(ALPHANUM_UNDER), identifier), |x: &str| {
                x == s
            }),
        )(i)?;

        let (after_line, (rest_of_line, _)) = many_till(anychar, eol)(rest)?;
        let rest_of_line = rest_of_line.iter().collect::<String>();

        let y: ParserResult<Option<(Vec<char>, &str)>> =
            opt(many_till(anychar, tag(" = ")))(rest_of_line.as_str());

        if let Ok((_, Some(_))) = y {
            return success(true)(after_line);
        }

        return success(false)(rest);
    }
}

pub fn decl_is_reassigned(i: &str) -> ParserResult<bool> {
    let (rest, name) = map(get_named_var, |x| x)(i)?;

    let z: ParserResult<bool> = map(many0(search_identifier_assignment(&name)), |x| {
        x.iter().any(|x| *x)
    })(rest);
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
