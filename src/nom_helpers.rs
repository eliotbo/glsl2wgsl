//! Various nom parser helpers.

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while1};
use nom::character::complete::{anychar, char, line_ending, multispace0, multispace1};
// use nom::combinator::{map, recognize, value, verify};
use nom::combinator::{cut, eof, map, peek, recognize, value, verify};
// use nom::error::{ErrorKind, ParseError, VerboseError, VerboseErrorKind};
use nom::error::{ErrorKind, VerboseError, VerboseErrorKind};
use nom::multi::{fold_many0, many0, many_till, separated_list0};
use nom::{Err as NomErr, IResult};

use nom::{
    character::complete::{alpha1, alphanumeric1},
    sequence::{delimited, pair},
    Parser,
};

use nom_locate::LocatedSpan;
// use nom::error::ParseError;

pub type Span<'a> = LocatedSpan<&'a str>;

pub const ALPHANUM_UNDER: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789";

pub type ParserResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;
pub type ParserResult2<'a, O> = IResult<Span<'a>, O, VerboseError<&'a str>>;

pub type IResult2<'a, O> = nom::IResult<Span<'a>, O, ParseError<'a>>;

#[derive(Debug, PartialEq)]
pub struct ParseError<'a> {
    span: Span<'a>,
    message: Option<String>,
}

impl<'a> ParseError<'a> {
    pub fn new(message: String, span: Span<'a>) -> Self {
        Self {
            span,
            message: Some(message),
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn line(&self) -> u32 {
        self.span().location_line()
    }

    pub fn offset(&self) -> usize {
        self.span().location_offset()
    }
}
// That's what makes it nom-compatible.
impl<'a> nom::error::ParseError<Span<'a>> for ParseError<'a> {
    fn from_error_kind(input: Span<'a>, kind: nom::error::ErrorKind) -> Self {
        Self::new(format!("parse error {:?}", kind), input)
    }

    fn append(_input: Span<'a>, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }

    fn from_char(input: Span<'a>, c: char) -> Self {
        Self::new(format!("unexpected character '{}'", c), input)
    }
}

// A constant parser that just forwards the value it’s parametered with without reading anything
// from the input. Especially useful as “fallback” in an alternative parser.
//
pub fn cnst<'a, T, E>(t: T) -> impl FnMut(&'a str) -> Result<(&'a str, T), E>
where
    T: 'a + Clone,
{
    move |i| Ok((i, t.clone()))
}

pub fn cnst_span<'a, T, E>(t: T) -> impl FnMut(Span<'a>) -> Result<(Span<'a>, T), E>
where
    T: 'a + Clone,
{
    move |i| Ok((i, t.clone()))
}

// End-of-input parser.
//
// Yields `()` if the parser is at the end of the input; an error otherwise.
// pub fn eoi(i: &str) -> ParserResult<()> {
pub fn eoi(i: &str) -> ParserResult<()> {
    if i.is_empty() {
        Ok((i, ()))
    } else {
        Err(NomErr::Error(VerboseError {
            errors: vec![(i, VerboseErrorKind::Nom(ErrorKind::Eof))],
        }))
    }
}

// End-of-input parser.
//
// Yields `()` if the parser is at the end of the input; an error otherwise.
// pub fn eoi2(i: &str) -> nom::IResult<Span, (), VerboseError<&str>> {
pub fn eoi_span(i: Span) -> IResult2<()> {
    if i.is_empty() {
        Ok((i, ()))
    } else {
        Err(nom::Err::Error(ParseError::new(
            "vector_selector must start from a metric name".to_owned(),
            i,
        )))
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

pub fn eol_span(i: Span) -> IResult2<()> {
    let (rest, m) = alt((
        eoi_span, // this one goes first because it’s very cheap
        value((), line_ending),
    ))(i)?;
    Ok((rest, m))
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

// Apply the `f` parser until `g` succeeds. Both parsers consume the input.
pub fn till_span<'a, A, B, F, G>(mut f: F, mut g: G) -> impl FnMut(Span<'a>) -> IResult2<'a, ()>
where
    F: FnMut(Span<'a>) -> IResult2<'a, A>,
    G: FnMut(Span<'a>) -> IResult2<'a, B>,
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

// A version of many0 that discards the result of the parser, preventing allocating.
pub fn many0_span_disc<'a, A, F>(mut f: F) -> impl FnMut(Span<'a>) -> IResult2<'a, ()>
where
    F: FnMut(Span<'a>) -> IResult2<'a, A>,
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

/// Parse a string until the end of line.
///
/// This parser accepts the multiline annotation (\) to break the string on several lines.
///
/// Discard any leading newline.
pub fn str_till_eol_span(i: Span) -> IResult2<Span> {
    let parser = recognize(till_span(
        alt((value((), tag("\\\n")), value((), anychar))),
        eol_span,
    ));
    map(parser, |ii| {
        let w = ii;
        if w.as_bytes().last() == Some(&b'\n') {
            LocatedSpan::new(&w[0..w.len() - 1])
        } else {
            w
        }
    })(i)
}

// Blank base parser.
//
// This parser succeeds with multispaces and multiline annotation.
pub fn blank_space(i: &str) -> ParserResult<&str> {
    recognize(many0_(alt((multispace1, tag("\\\n")))))(i)
}

pub fn blank_space_span(i: Span) -> IResult2<Span> {
    recognize(many0_span_disc(alt((multispace1, tag("\\\n")))))(i)
}

// pub fn blank_space(i: &str) -> ParserResult<String> {
//     map(recognize(many0(alt((multispace0, tag("\\\n"))))), |_x| {
//         "".to_string()
//     })(i)
// }

pub fn blank_space2(i: &str) -> ParserResult<String> {
    map(many0(alt((multispace0, tag("\t")))), |_x| "".to_string())(i)
}

/// Parse a path literal with double quotes.
pub fn path_lit_relative(i: Span) -> IResult2<Span> {
    map(
        delimited(char('"'), cut(take_until("\"")), cut(char('"'))),
        |s: Span| s,
    )(i)
}

#[inline]
pub fn identifier_pred(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

pub fn identifier_num_pred(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == '.'
}

fn func_pred(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == '(' || ch == ' ' || ch == ')' || ch == ','
}

#[inline]
pub fn verify_identifier(s: &Span) -> bool {
    !char::from(s.fragment().as_bytes()[0]).is_digit(10)
}

fn identifier_hashtag(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ch == '#'
}

pub fn anychar_func(i: &str) -> ParserResult<String> {
    map(take_while1(func_pred), |v: &str| v.to_string())(i)
}

/// Parse an identifier (raw version).
fn _identifier_str(i: Span) -> IResult2<Span> {
    verify(take_while1(identifier_pred), verify_identifier)(i)
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

pub fn anychar_underscore_dot(i: &str) -> ParserResult<String> {
    map(take_while1(identifier_num_pred), |v: &str| v.to_string())(i)
}

pub fn anychar_underscore(i: &str) -> ParserResult<String> {
    map(take_while1(identifier_num_pred), |v: &str| v.to_string())(i)
}

pub fn anychar_underscore_hashtag(i: &str) -> ParserResult<String> {
    map(take_while1(identifier_hashtag), |v: &str| v.to_string())(i)
}

pub fn rest_of_script(i: &str) -> ParserResult<String> {
    map(many_till(anychar, eof), |x| x.0.iter().collect())(i)
}

// search (v, where v is an identifier) and replace by (num, which can be anychar)
pub fn search_and_replace_identifier(i: &str, v: String, num: String) -> ParserResult<String> {
    map(
        many_till(
            many_till(anychar, alt((verify(identifier, |x| x == v), eof))),
            eof,
        ),
        |x| {
            // makes sure that the identifier does not have any other alphanum characters
            // before and after it
            let mut ret = "".to_string();
            for (v_chars, name) in x.0.iter() {
                ret.push_str(&v_chars.iter().collect::<String>());
                if let Some(c) = ret.chars().last() {
                    if name == &v && !c.is_alphanumeric() {
                        ret.push_str(&num);
                    } else {
                        ret.push_str(&name);
                    }
                } else {
                    ret.push_str(&name);
                }
            }

            ret
        },
    )(i)
}

// search (v, where v is an identifier) and replace by (num, which can be anychar)
pub fn search_and_replace(i: &str, v: String, num: String) -> ParserResult<String> {
    map(
        many_till(
            many_till(
                anychar,
                alt((
                    verify(anychar_underscore_hashtag, |x: &str| x.to_string() == v),
                    map(eof, |x: &str| x.to_string()),
                )),
            ),
            eof,
        ),
        |x| {
            //

            let mut ret = "".to_string();
            for (v_chars, name) in x.0.iter() {
                ret.push_str(&v_chars.iter().collect::<String>());
                if name == &v {
                    ret.push_str(&num);
                }
            }
            ret
        },
    )(i)
}

pub fn till_next_paren_or_comma(i: &str) -> ParserResult<(String, String)> {
    map(
        many_till(anychar, peek(alt((tag("("), tag(")"), tag(","))))),
        |(so_far, brack): (Vec<char>, &str)| {
            //
            let text = so_far.iter().collect::<String>();
            return (text, brack.to_string());
        },
    )(i)
}

// parse one argument of a function call
pub fn argument1(i: &str) -> ParserResult<String> {
    let mut parsed_text: String = "".to_string();
    let mut scope = 0;
    let mut rest = i;
    loop {
        let (rest1, (text_so_far, paren_or_comma)): (&str, (String, String)) =
            till_next_paren_or_comma(rest)?;
        rest = rest1;
        parsed_text += &text_so_far;

        // println!("scope: {:?}", scope);
        match paren_or_comma.as_str() {
            "(" => {
                let (rest1, _single) = char('(')(rest)?;
                rest = rest1;
                scope += 1;
                parsed_text += &paren_or_comma;
            }
            ")" => {
                scope -= 1;

                // end of function call
                if scope == -1 {
                    break;
                } else {
                    let (rest1, _single) = char(')')(rest)?;
                    rest = rest1;
                    parsed_text += &paren_or_comma;
                }
            }
            _ => {
                // case of a comma
                // end of argument

                if scope == 0 {
                    break;
                } else {
                    let (rest1, _single) = char(',')(rest)?;
                    rest = rest1;
                    parsed_text += &paren_or_comma;
                }
            }
        }

        // for any non-breaking char, parse it so we can get to the next ")", "(", or ","
        // let (rest1, _char) = anychar(rest)?;
        // rest = rest1;

        // println!("parsed_text: {:?}", parsed_text);
    }
    // println!("parsed_text: {:?}", parsed_text);

    Ok((rest, parsed_text))
}

pub fn till_next_bracket_or_comma(i: &str) -> ParserResult<(String, String)> {
    map(
        many_till(anychar, peek(alt((tag("<"), tag(">"), tag(")"), tag(","))))),
        |(so_far, brack): (Vec<char>, &str)| {
            //
            let text = so_far.iter().collect::<String>();
            return (text, brack.to_string());
        },
    )(i)
}

// parse one argument of a function call
pub fn type_argument1(i: &str) -> ParserResult<String> {
    let mut parsed_text: String = "".to_string();
    let mut scope = 0;
    let mut rest = i;
    loop {
        let (rest1, (text_so_far, paren_or_comma)): (&str, (String, String)) =
            till_next_bracket_or_comma(rest)?;
        rest = rest1;
        parsed_text += &text_so_far;

        // println!("scope: {:?}", scope);
        match paren_or_comma.as_str() {
            "<" => {
                let (rest1, _single) = char('<')(rest)?;
                rest = rest1;
                scope += 1;
                parsed_text += &paren_or_comma;
            }
            ">" => {
                scope -= 1;

                // end of function call
                if scope == -1 {
                    break;
                } else {
                    let (rest1, _single) = char('>')(rest)?;
                    rest = rest1;
                    parsed_text += &paren_or_comma;
                }
            }
            _ => {
                // case of a comma or a
                // end of argument

                if scope == 0 {
                    break;
                } else {
                    let (rest1, _single) = char(',')(rest)?;
                    rest = rest1;
                    parsed_text += &paren_or_comma;
                }
            }
        }

        // for any non-breaking char, parse it so we can get to the next ")", "(", or ","
        // let (rest1, _char) = anychar(rest)?;
        // rest = rest1;

        // println!("parsed_text: {:?}", parsed_text);
    }
    // println!("parsed_text: {:?}", parsed_text);

    Ok((rest, parsed_text))
}

pub fn function_call_args_anychar(i: &str) -> ParserResult<Vec<String>> {
    map(
        // preceded(
        //     tag("("),
        //     many0(delimited(multispace0, argument1, multispace0)),
        // ),
        // preceded(tag("("), many0(preceded(multispace0, argument1))),
        // delimited(
        //     tag("("),
        //     many0(delimited(multispace0, argument1, multispace0)),
        //     tag(")"),
        // ),
        delimited(
            tag("("),
            separated_list0(tag(","), delimited(multispace0, argument1, multispace0)),
            tag(")"),
        ),
        |x: Vec<String>| {
            //

            x
        },
    )(i)
}

pub fn function_call_args(i: &str) -> ParserResult<Vec<String>> {
    map(
        delimited(
            tag("("),
            separated_list0(
                delimited(multispace0, char(','), multispace0),
                anychar_underscore_dot,
            ),
            tag(")"),
        ),
        |x| {
            // println!("x: {:?}", x);
            x
        },
    )(i)
}

pub fn till_next_brace(i: &str) -> ParserResult<(String, String)> {
    map(
        many_till(anychar, alt((tag("{"), tag("}")))),
        |(so_far, brack): (Vec<char>, &str)| {
            //

            let text = so_far.iter().collect::<String>();
            return (text, brack.to_string());
        },
    )(i)
}

pub fn get_function_body(i: &str, mut scope: u32) -> ParserResult<String> {
    let mut parsed_text: String = "".to_string();
    let mut rest = i;
    loop {
        let (rest1, (text_so_far, brace)): (&str, (String, String)) = till_next_brace(rest)?;
        rest = rest1;
        parsed_text += &text_so_far;
        parsed_text += &brace;

        if brace == "{" {
            scope += 1;
        } else {
            scope -= 1;
        }

        if scope == 0 {
            break;
        }
    }

    return Ok((rest, (parsed_text)));
}
