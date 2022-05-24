//! Various nom parser helpers.

use nom::branch::{alt, Alt};
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{anychar, line_ending, multispace1};
use nom::combinator::{map, recognize, value};
// use nom::error::{ErrorKind, ParseError, VerboseError, VerboseErrorKind};
use nom::error::{ErrorKind, VerboseError, VerboseErrorKind};
use nom::multi::fold_many0;
use nom::{Err as NomErr, IResult};

use nom::{
    character::complete::{alpha1, alphanumeric1},
    multi::many0,
    sequence::pair,
};

use nom_locate::{position, LocatedSpan};
// use nom::error::ParseError;

pub type Span<'a> = LocatedSpan<&'a str>;

struct Token<'a> {
    pub position: Span<'a>,
    pub foo: &'a str,
    pub bar: &'a str,
}

fn parse_foobar(s: Span) -> IResult<Span, Token> {
    let (s, _) = take_until("foo")(s)?;
    let (s, pos) = position(s)?;
    let (s, foo) = tag("foo")(s)?;
    let (s, bar) = tag("bar")(s)?;

    Ok((
        s,
        Token {
            position: pos,
            foo: foo.fragment(),
            bar: bar.fragment(),
        },
    ))
}

fn aw() {
    let input = Span::new("Lorem ipsum \n foobar");
    let output = parse_foobar(input);
    let position = output.unwrap().1.position;
    assert_eq!(position.location_offset(), 14);
    assert_eq!(position.location_line(), 2);
    assert_eq!(position.fragment(), &"");
    assert_eq!(position.get_column(), 2);
}

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

fn identifier(input: Span) -> IResult<Span, Token> {
    // [a-zA-Z_][a-zA-Z0-9_]*
    let (rest, m) = recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)?;

    let (s, pos) = position(rest)?;
    let (s, foo) = tag("foo")(rest)?;
    let (s, bar) = tag("bar")(rest)?;

    let token = Token {
        position: pos,
        foo: &foo,
        bar: &bar,
    };
    Ok((rest, token))
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
pub fn many0__span<'a, A, F>(mut f: F) -> impl FnMut(Span<'a>) -> IResult2<'a, ()>
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
        let mut w = ii;
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
//
// Taylor Swift loves it.
pub fn blank_space(i: &str) -> ParserResult<&str> {
    recognize(many0_(alt((multispace1, tag("\\\n")))))(i)
}

pub fn blank_space_span(i: Span) -> IResult2<Span> {
    recognize(many0__span(alt((multispace1, tag("\\\n")))))(i)
}
