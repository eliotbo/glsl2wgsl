// This parser finds function arguments that are reassigned within the function body.
// If so, the parser creates new variables preceded by the "var" keyword and changes
// all occurences of the argument in the function body by the new variable

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{anychar, char, multispace0, one_of};
use nom::combinator::{eof, map, opt, peek, recognize, success, verify};
use nom::multi::{many0, many_till, separated_list0};
use nom::sequence::{delimited, pair, preceded};
use nom::Parser;

pub use crate::nom_helpers::*;

pub fn argument1_simple(i: &str) -> ParserResult<String> {
    map(many_till(anychar, peek(one_of(",)"))), |x| {
        x.0.iter().collect::<String>()
        // &s
    })(i)
}

pub fn function_call_args_anychar2(i: &str) -> ParserResult<Vec<String>> {
    map(
        delimited(
            tag("("),
            separated_list0(
                delimited(multispace0, char(','), multispace0),
                alt((argument1_simple, map(multispace0, |x: &str| x.to_string()))),
            ),
            tag(")"),
        ),
        |x: Vec<String>| x,
    )(i)
}

pub fn get_arg_name(i: &str) -> ParserResult<String> {
    map(many_till(anychar, tag(":")), |x: (Vec<char>, &str)| {
        x.0.iter().collect::<String>()
    })(i)
}

pub fn search_for_full_identifier<'a, 'b>(
    s: &'b str,
) -> impl FnMut(&'a str) -> ParserResult<bool> + 'b {
    move |i: &str| {
        let x = opt(many_till(
            anychar,
            verify(preceded(is_not(ALPHANUM_UNDER), identifier), |x: &str| {
                x == s
            })
            .and(alt((
                tag(" = "),
                tag(" += "),
                tag(" -= "),
                tag(" *= "),
                tag(" /= "),
                tag(" %= "),
            ))),
        ))(i)?;

        if let (rest, Some((_, _name))) = x {
            return success(true)(rest);
        }

        // if let (rest, Some((_, _name))) = x {
        //     let (rest2, (rest_of_line, _)) = many_till(anychar, eol)(rest)?;
        //     let rest_of_line = rest_of_line.iter().collect::<String>();

        //     let y: ParserResult<Option<(Vec<char>, &str)>> =
        //         opt(many_till(anychar, tag(" = ")))(rest_of_line.as_str());

        //     if let Ok((_, Some(_))) = y {
        //         return success(true)(rest2);
        //     }
        // }

        return success(false)(i);
    }
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

pub fn get_named_var(i: &str) -> ParserResult<String> {
    map(
        tag("let ").and(till_space_or_colon).and(either_type_or_not),
        |(x1, _x2)| x1.1,
    )(i)
}

pub fn decl_is_reassigned(i: &str, name: String) -> ParserResult<bool> {
    // let (rest, name) = map(get_named_var, |x| x)(i)?;

    let z: ParserResult<bool> = map(peek(search_for_full_identifier(&name)), |x| x)(i);
    return z;
}

pub fn check_one_func(i: &str) -> ParserResult<String> {
    // let (_, arguments) = map(
    //     peek(many_till(
    //         anychar,
    //         preceded(tag("fn "), identifier).and(function_call_args_anychar2),
    //     )),
    //     |(_s, (_iden, args))| args,
    // )(i)?;

    // delete "inout" keywords and add type ptr<function, particle>,
    let (_, args) = map(
        peek(many_till(
            anychar,
            pair(tag("fn "), identifier).and(function_call_args_anychar2),
        )),
        |(_before_func, ((_fn_tag, _func_name), args)): (
            Vec<char>,
            ((&str, &str), Vec<String>),
        )| {
            //
            // println!("args: {:?}", args);

            let args = args
                .iter()
                .map(|full_arg| {
                    if let Ok((arg_type, arg_name)) = get_arg_name(full_arg) {
                        // do not create a new var if the argument is a pointer
                        if arg_type.starts_with(" ptr") {
                            "".to_string()
                        } else {
                            arg_name
                        }
                    } else {
                        "".to_string()
                    }
                })
                .collect::<Vec<String>>();

            return args;
        },
    )(i)?;

    // // if there are no arguments, do not alter the function
    // if args == vec!["".to_string()] {
    //     return Ok(("", i.to_string()));
    // }

    let (_rest, _function_decl) = recognize(many_till(
        anychar,
        pair(tag("fn "), identifier).and(function_call_args_anychar2),
    ))(i)?;

    // let mut body_removed_curly = (&body.clone()[3..]).to_string();

    let (fn_declaration, rest_removed_curly) = i
        .split_once("{")
        .expect("could not find opening curly brace");
    // let mut body_with_curly = "{".to_string() + body_removed_curly;

    let (rest, mut body_removed_curly) = get_function_body(rest_removed_curly, 1)?;

    // println!("{}", body_removed_curly);

    for arg in args.iter() {
        // if the function body contains a reassigned for "arg" of the type "arg = ...",
        // crate a new variable with a the new ending "_var" and replace all instances of
        // "arg" by "arg"_var

        if let Ok((_, true)) = decl_is_reassigned(&body_removed_curly, arg.to_string()) {
            // println!("arg: {:?}", arg);
            let new_arg = arg.clone() + "_var";
            if let Ok((_, body2)) =
                search_and_replace_identifier(&body_removed_curly, arg.clone(), new_arg)
            {
                let new_var = format!("\n\tvar {}_var = {};", arg, arg).to_string();
                // body = new_var + &body2;
                body_removed_curly = new_var + &body2;
            }
        }
    }

    // println!("body: {:?}", body_removed_curly);

    return Ok((
        rest,
        // function_decl.to_string() + "  {" + &body_removed_curly,
        fn_declaration.to_string() + "{" + &body_removed_curly,
    ));
}

pub fn add_var_to_reassigned_args(i: &str) -> ParserResult<String> {
    map(
        many0(check_one_func).and(many_till(anychar, eof)),
        |(vec_parsed, rest)| vec_parsed.join("") + &rest.0.iter().collect::<String>(),
    )(i)
}
