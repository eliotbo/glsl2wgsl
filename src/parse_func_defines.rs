// This parser parses the "#define func(arg0, arg1) other_func(arg0, arg1)" statements,
// and replaces all instances of func() with other_func().

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, multispace0, none_of, space1};
use nom::combinator::{eof, map, success, verify};
use nom::error::VerboseError;
use nom::multi::{count, many0, many_till};
use nom::sequence::{pair, preceded};

use nom::IResult;
use nom::Parser;

use crate::nom_helpers::*;

pub type ParserResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

pub fn erase_one_define(i: &str) -> ParserResult<String> {
    map(
        many_till(
            anychar,
            preceded(
                tag("#define ")
                    .and(anychar_underscore_dot)
                    .and(function_call_args),
                many_till(anychar, tag("\n")),
            ),
        ),
        |x| {
            // println!("x: {:?}", x.0.iter().collect::<String>());
            x.0.iter().collect()
        },
    )(i)
}

// pub fn rest_of_script(i: &str) -> ParserResult<String> {
//     map(many_till(anychar, eof), |x| x.0.iter().collect())(i)
// }

pub fn erase_all_func_defines(i: &str) -> ParserResult<String> {
    map(
        many_till(alt((erase_one_define, rest_of_script)), eof),
        |x| x.0.join(""),
    )(i)
}

pub fn replace_define_tag(i: &str) -> ParserResult<(String, Vec<String>)> {
    map(
        preceded(tag("#define").and(space1), anychar_underscore_dot).and(function_call_args),
        |x| {
            // println!("x: {:?}", x);
            x
        },
    )(i)
}

#[derive(Debug, Clone)]
pub struct DefineFunc {
    pub name: String,
    pub args: Vec<String>,
    pub replace_by: String,
}

pub fn get_name_and_args(i: &str) -> ParserResult<(String, Vec<String>)> {
    map(
        many_till(anychar, replace_define_tag),
        |(_v, (name, args))| (name, args),
    )(i)
}

// TODO: right now, the end of a defined function is detected with a newline char.
// Perhaps make more robust...
pub fn get_assignment(i: &str) -> ParserResult<String> {
    map(preceded(multispace0, many_till(anychar, tag("\n"))), |x| {
        x.0.iter().collect()
    })(i)
}

// parsing of a #define function
// e.g. #define texel(buffer, position) texelFetch(buffer, position, 0)
pub fn construct_assignment_vars(i: &str) -> ParserResult<DefineFunc> {
    let (rest, (name, args)) = get_name_and_args(i)?;
    let (rest, mut assignment) = get_assignment(rest)?;

    for (num, arg) in args.iter().enumerate() {
        let num_str = "#arg_".to_string() + &num.to_string();
        // println!("arg: {}", arg);
        if let Ok((_, assignment2)) =
            search_and_replace_identifier(&assignment, arg.to_string(), num_str)
        {
            // println!("assignment2assignment2assignment2",);
            assignment = assignment2;
        }
    }

    let define_func = DefineFunc {
        name,
        args,
        replace_by: assignment,
    };

    return Ok((rest, define_func));
}

pub fn get_all_define_funcs(i: &str) -> ParserResult<Vec<DefineFunc>> {
    map(many0(construct_assignment_vars), |x2| x2)(i)
}

pub fn detect_identifier_as_arg(i: &str, name: String) -> ParserResult<String> {
    verify(anychar_underscore_dot, |x: &str| x.to_string() == name)(i)
}

// finds an instance of the #define function in the code body and replaces it
// by its definition using the parsed arguments
pub fn find_and_replace_single_define_func(i: &str, def: DefineFunc) -> ParserResult<String> {
    map(
        many0(
            many_till(
                anychar,
                pair(
                    // the character before the name of the function should not be
                    // part of the name
                    count(none_of(ALPHANUM_UNDER), 1),
                    verify(anychar_underscore_dot, |x: &str| {
                        //

                        x.to_string() == def.name
                    }),
                ),
            )
            .and(function_call_args_anychar),
        )
        .and(rest_of_script),
        |(lines, rest)| {
            let mut all_script = "".to_string();
            //

            // for each instance of the #define function in the code body
            for ((so_far_chars, (single_char, _name)), args) in lines.iter() {
                //
                let mut so_far = so_far_chars.iter().collect::<String>();
                so_far += &single_char.iter().collect::<String>();

                // println!("lines : {:?} --> {:?} --> {:?}", so_far, _name, args);

                let mut replaced_expression = def.replace_by.to_string();
                //
                // replace the placeholder argument by the ones provided in the code body
                for (n, arg) in args.iter().enumerate() {
                    //
                    let num_str = "#arg_".to_string() + &n.to_string();
                    if let Ok((_, assignment)) =
                        search_and_replace(&replaced_expression, num_str.clone(), arg.to_string())
                    {
                        replaced_expression = assignment;
                    }
                }

                so_far.push_str(&replaced_expression);
                all_script.push_str(&so_far);
            }
            all_script.push_str(&rest);
            all_script
        },
    )(i)
}

pub fn find_and_replace_define_funcs(i: &str, defs: Vec<DefineFunc>) -> ParserResult<String> {
    let mut full_script = i.to_string();

    for def in defs.iter() {
        if let Ok((_, fs)) = find_and_replace_single_define_func(&full_script, def.clone()) {
            full_script = fs;
        }
    }

    success(full_script)("")
}

// counts the number of #defines instances in the script
pub fn count_defines(i: &str) -> ParserResult<usize> {
    map(
        many_till(
            many_till(
                anychar,
                alt((
                    verify(anychar_underscore_hashtag, |x: &str| x == "#define"),
                    map(eof, |x: &str| x.to_string()),
                )),
            ),
            eof,
        ),
        |x| {
            //
            // println!("{:?}", x.0.len());
            x.0.len() - 1
        },
    )(i)
}

pub fn func_definition_parser(i: &str) -> ParserResult<(String, u32)> {
    let (_, number_of_defines) = count_defines(i)?;

    let (_rest, define_funcs) = get_all_define_funcs(i)?;

    let (_, no_defines) = erase_all_func_defines(i)?;

    // println!("no_defines : {:?}", define_funcs);

    if let Ok((_rest, so_far)) = find_and_replace_define_funcs(&no_defines, define_funcs) {
        return success((so_far, number_of_defines as u32))("");
    }

    success(("".to_string(), 0))("")
}

// // pub fn remove_comments(i: &str) -> ParserResult<String> {
// //     map(
// //         many0(many_till(anychar, tag("//").and(many_till(anychar, eol)))),
// //         |x| {
// //             let y = x
// //                 .iter()
// //                 .map(|z| z.0.iter().collect::<String>())
// //                 .collect::<Vec<String>>();
// //             y.join("\n")
// //         },
// //     )(i)
// // }

// pub fn func_definition_parser(i: &str) -> ParserResult<String> {
//     // let mut ii = i.to_string();

//     // if let Ok((_, removed_comments)) = remove_comments(i) {
//     //     ii = removed_comments;
//     // }

//     // let rc = removed_comments.clone();

//     let (_rest, define_funcs) = get_all_define_funcs(i)?;

//     let (_, no_defines) = erase_all_func_defines(i)?;

//     // println!("no_defines : {:?}", define_funcs);

//     if let Ok((_rest, so_far)) = find_and_replace_define_funcs(&no_defines, define_funcs) {
//         return success(so_far.clone())("");
//     }

//     success("".to_string())("")
// }
