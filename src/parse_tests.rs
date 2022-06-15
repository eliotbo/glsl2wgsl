use crate::parsers_span::*;
use crate::syntax;

#[test]
fn parse_uniline_comment() {
    assert_eq!(
        comment(Span::new("// lol")),
        Ok((Span::new(""), Span::new(" lol")))
    );
    assert_eq!(
        comment(Span::new("// lol\nfoo")),
        Ok((Span::new("foo"), Span::new(" lol")))
    );
    assert_eq!(
        comment(Span::new("// lol\\\nfoo")),
        Ok((Span::new(""), Span::new(" lol\\\nfoo")))
    );
    assert_eq!(
        comment(Span::new("// lol   \\\n   foo\n")),
        Ok((Span::new(""), Span::new(" lol   \\\n   foo")))
    );
}

#[test]
fn parse_multiline_comment() {
    assert_eq!(
        comment(Span::new("/* lol\nfoo\n*/bar")),
        Ok((Span::new("bar"), Span::new(" lol\nfoo\n")))
    );
}

#[test]
fn parse_unsigned_suffix() {
    assert_eq!(unsigned_suffix(Span::new("u")), Ok((Span::new(""), 'u')));
    assert_eq!(unsigned_suffix(Span::new("U")), Ok((Span::new(""), 'U')));
}

// #[test]
// fn parse_nonzero_digits() {
//     assert_eq!(
//         nonzero_digits(Span::new("3")),
//         Ok((Span::new(""), Span::new("3")))
//     );
//     assert_eq!(
//         nonzero_digits(Span::new("12345953")),
//         Ok((Span::new(""), Span::new("12345953")))
//     );
// }

#[test]
fn parse_decimal_lit() {
    assert_eq!(decimal_lit(Span::new("3")), Ok((Span::new(""), Ok(3))));
    assert_eq!(decimal_lit(Span::new("3")), Ok((Span::new(""), Ok(3))));
    assert_eq!(decimal_lit(Span::new("13")), Ok((Span::new(""), Ok(13))));
    assert_eq!(decimal_lit(Span::new("42")), Ok((Span::new(""), Ok(42))));
    assert_eq!(
        decimal_lit(Span::new("123456")),
        Ok((Span::new(""), Ok(123456)))
    );
}

#[test]
fn parse_octal_lit() {
    assert_eq!(octal_lit(Span::new("0")), Ok((Span::new(""), Ok(0o0))));
    assert_eq!(octal_lit(Span::new("03 ")), Ok((Span::new(" "), Ok(0o3))));
    assert_eq!(octal_lit(Span::new("012 ")), Ok((Span::new(" "), Ok(0o12))));
    assert_eq!(
        octal_lit(Span::new("07654321 ")),
        Ok((Span::new(" "), Ok(0o7654321)))
    );
}

#[test]
fn parse_hexadecimal_lit() {
    assert_eq!(
        hexadecimal_lit(Span::new("0x3 ")),
        Ok((Span::new(" "), Ok(0x3)))
    );
    assert_eq!(
        hexadecimal_lit(Span::new("0x0123789")),
        Ok((Span::new(""), Ok(0x0123789)))
    );
    assert_eq!(
        hexadecimal_lit(Span::new("0xABCDEF")),
        Ok((Span::new(""), Ok(0xabcdef)))
    );
    assert_eq!(
        hexadecimal_lit(Span::new("0xabcdef")),
        Ok((Span::new(""), Ok(0xabcdef)))
    );
}

#[test]
fn parse_integral_lit() {
    assert_eq!(integral_lit(Span::new("0")), Ok((Span::new(""), 0)));
    assert_eq!(integral_lit(Span::new("3")), Ok((Span::new(""), 3)));
    assert_eq!(integral_lit(Span::new("3 ")), Ok((Span::new(" "), 3)));
    assert_eq!(integral_lit(Span::new("03 ")), Ok((Span::new(" "), 3)));
    assert_eq!(
        integral_lit(Span::new("076556 ")),
        Ok((Span::new(" "), 0o76556))
    );
    assert_eq!(integral_lit(Span::new("012 ")), Ok((Span::new(" "), 0o12)));
    assert_eq!(integral_lit(Span::new("0x3 ")), Ok((Span::new(" "), 0x3)));
    assert_eq!(
        integral_lit(Span::new("0x9ABCDEF")),
        Ok((Span::new(""), 0x9ABCDEF))
    );
    assert_eq!(
        integral_lit(Span::new("0x9ABCDEF")),
        Ok((Span::new(""), 0x9ABCDEF))
    );
    assert_eq!(
        integral_lit(Span::new("0x9abcdef")),
        Ok((Span::new(""), 0x9abcdef))
    );
    assert_eq!(
        integral_lit(Span::new("0x9abcdef")),
        Ok((Span::new(""), 0x9abcdef))
    );
    assert_eq!(
        integral_lit(Span::new("0xffffffff")),
        Ok((Span::new(""), 0xffffffffu32 as i32))
    );
}

#[test]
fn parse_integral_neg_lit() {
    assert_eq!(integral_lit(Span::new("-3")), Ok((Span::new(""), -3)));
    assert_eq!(integral_lit(Span::new("-3 ")), Ok((Span::new(" "), -3)));
    assert_eq!(integral_lit(Span::new("-03 ")), Ok((Span::new(" "), -3)));
    assert_eq!(
        integral_lit(Span::new("-076556 ")),
        Ok((Span::new(" "), -0o76556))
    );
    assert_eq!(
        integral_lit(Span::new("-012 ")),
        Ok((Span::new(" "), -0o12))
    );
    assert_eq!(integral_lit(Span::new("-0x3 ")), Ok((Span::new(" "), -0x3)));
    assert_eq!(
        integral_lit(Span::new("-0x9ABCDEF")),
        Ok((Span::new(""), -0x9ABCDEF))
    );
    assert_eq!(
        integral_lit(Span::new("-0x9ABCDEF")),
        Ok((Span::new(""), -0x9ABCDEF))
    );
    assert_eq!(
        integral_lit(Span::new("-0x9abcdef")),
        Ok((Span::new(""), -0x9abcdef))
    );
    assert_eq!(
        integral_lit(Span::new("-0x9abcdef")),
        Ok((Span::new(""), -0x9abcdef))
    );
}

#[test]
fn parse_unsigned_lit() {
    assert_eq!(
        unsigned_lit(Span::new("0xffffffffU")),
        Ok((Span::new(""), 0xffffffff as u32))
    );
    assert_eq!(
        unsigned_lit(Span::new("-1u")),
        Ok((Span::new(""), 0xffffffff as u32))
    );
    assert!(unsigned_lit(Span::new("0xfffffffffU")).is_err());
}

#[test]
fn parse_float_lit() {
    assert_eq!(float_lit(Span::new("0.;")), Ok((Span::new(";"), 0.)));
    assert_eq!(float_lit(Span::new(".0;")), Ok((Span::new(";"), 0.)));
    assert_eq!(float_lit(Span::new(".035 ")), Ok((Span::new(" "), 0.035)));
    assert_eq!(float_lit(Span::new("0. ")), Ok((Span::new(" "), 0.)));
    assert_eq!(float_lit(Span::new("0.035 ")), Ok((Span::new(" "), 0.035)));
    assert_eq!(float_lit(Span::new(".035f")), Ok((Span::new(""), 0.035)));
    assert_eq!(float_lit(Span::new("0.f")), Ok((Span::new(""), 0.)));
    assert_eq!(float_lit(Span::new("314.f")), Ok((Span::new(""), 314.)));
    assert_eq!(float_lit(Span::new("0.035f")), Ok((Span::new(""), 0.035)));
    assert_eq!(float_lit(Span::new(".035F")), Ok((Span::new(""), 0.035)));
    assert_eq!(float_lit(Span::new("0.F")), Ok((Span::new(""), 0.)));
    assert_eq!(float_lit(Span::new("0.035F")), Ok((Span::new(""), 0.035)));
    assert_eq!(
        float_lit(Span::new("1.03e+34 ")),
        Ok((Span::new(" "), 1.03e+34))
    );
    assert_eq!(
        float_lit(Span::new("1.03E+34 ")),
        Ok((Span::new(" "), 1.03E+34))
    );
    assert_eq!(
        float_lit(Span::new("1.03e-34 ")),
        Ok((Span::new(" "), 1.03e-34))
    );
    assert_eq!(
        float_lit(Span::new("1.03E-34 ")),
        Ok((Span::new(" "), 1.03E-34))
    );
    assert_eq!(
        float_lit(Span::new("1.03e+34f")),
        Ok((Span::new(""), 1.03e+34))
    );
    assert_eq!(
        float_lit(Span::new("1.03E+34f")),
        Ok((Span::new(""), 1.03E+34))
    );
    assert_eq!(
        float_lit(Span::new("1.03e-34f")),
        Ok((Span::new(""), 1.03e-34))
    );
    assert_eq!(
        float_lit(Span::new("1.03E-34f")),
        Ok((Span::new(""), 1.03E-34))
    );
    assert_eq!(
        float_lit(Span::new("1.03e+34F")),
        Ok((Span::new(""), 1.03e+34))
    );
    assert_eq!(
        float_lit(Span::new("1.03E+34F")),
        Ok((Span::new(""), 1.03E+34))
    );
    assert_eq!(
        float_lit(Span::new("1.03e-34F")),
        Ok((Span::new(""), 1.03e-34))
    );
    assert_eq!(
        float_lit(Span::new("1.03E-34F")),
        Ok((Span::new(""), 1.03E-34))
    );
}

#[test]
fn parse_float_neg_lit() {
    assert_eq!(float_lit(Span::new("-.035 ")), Ok((Span::new(" "), -0.035)));
    assert_eq!(float_lit(Span::new("-0. ")), Ok((Span::new(" "), -0.)));
    assert_eq!(
        float_lit(Span::new("-0.035 ")),
        Ok((Span::new(" "), -0.035))
    );
    assert_eq!(float_lit(Span::new("-.035f")), Ok((Span::new(""), -0.035)));
    assert_eq!(float_lit(Span::new("-0.f")), Ok((Span::new(""), -0.)));
    assert_eq!(float_lit(Span::new("-0.035f")), Ok((Span::new(""), -0.035)));
    assert_eq!(float_lit(Span::new("-.035F")), Ok((Span::new(""), -0.035)));
    assert_eq!(float_lit(Span::new("-0.F")), Ok((Span::new(""), -0.)));
    assert_eq!(float_lit(Span::new("-0.035F")), Ok((Span::new(""), -0.035)));
    assert_eq!(
        float_lit(Span::new("-1.03e+34 ")),
        Ok((Span::new(" "), -1.03e+34))
    );
    assert_eq!(
        float_lit(Span::new("-1.03E+34 ")),
        Ok((Span::new(" "), -1.03E+34))
    );
    assert_eq!(
        float_lit(Span::new("-1.03e-34 ")),
        Ok((Span::new(" "), -1.03e-34))
    );
    assert_eq!(
        float_lit(Span::new("-1.03E-34 ")),
        Ok((Span::new(" "), -1.03E-34))
    );
    assert_eq!(
        float_lit(Span::new("-1.03e+34f")),
        Ok((Span::new(""), -1.03e+34))
    );
    assert_eq!(
        float_lit(Span::new("-1.03E+34f")),
        Ok((Span::new(""), -1.03E+34))
    );
    assert_eq!(
        float_lit(Span::new("-1.03e-34f")),
        Ok((Span::new(""), -1.03e-34))
    );
    assert_eq!(
        float_lit(Span::new("-1.03E-34f")),
        Ok((Span::new(""), -1.03E-34))
    );
    assert_eq!(
        float_lit(Span::new("-1.03e+34F")),
        Ok((Span::new(""), -1.03e+34))
    );
    assert_eq!(
        float_lit(Span::new("-1.03E+34F")),
        Ok((Span::new(""), -1.03E+34))
    );
    assert_eq!(
        float_lit(Span::new("-1.03e-34F")),
        Ok((Span::new(""), -1.03e-34))
    );
    assert_eq!(
        float_lit(Span::new("-1.03E-34F")),
        Ok((Span::new(""), -1.03E-34))
    );
}

#[test]
fn parse_double_lit() {
    assert_eq!(double_lit(Span::new("0.;")), Ok((Span::new(";"), 0.)));
    assert_eq!(double_lit(Span::new(".0;")), Ok((Span::new(";"), 0.)));
    assert_eq!(double_lit(Span::new(".035 ")), Ok((Span::new(" "), 0.035)));
    assert_eq!(double_lit(Span::new("0. ")), Ok((Span::new(" "), 0.)));
    assert_eq!(double_lit(Span::new("0.035 ")), Ok((Span::new(" "), 0.035)));
    assert_eq!(double_lit(Span::new("0.lf")), Ok((Span::new(""), 0.)));
    assert_eq!(double_lit(Span::new("0.035lf")), Ok((Span::new(""), 0.035)));
    assert_eq!(double_lit(Span::new(".035lf")), Ok((Span::new(""), 0.035)));
    assert_eq!(double_lit(Span::new(".035LF")), Ok((Span::new(""), 0.035)));
    assert_eq!(double_lit(Span::new("0.LF")), Ok((Span::new(""), 0.)));
    assert_eq!(double_lit(Span::new("0.035LF")), Ok((Span::new(""), 0.035)));
    assert_eq!(
        double_lit(Span::new("1.03e+34lf")),
        Ok((Span::new(""), 1.03e+34))
    );
    assert_eq!(
        double_lit(Span::new("1.03E+34lf")),
        Ok((Span::new(""), 1.03E+34))
    );
    assert_eq!(
        double_lit(Span::new("1.03e-34lf")),
        Ok((Span::new(""), 1.03e-34))
    );
    assert_eq!(
        double_lit(Span::new("1.03E-34lf")),
        Ok((Span::new(""), 1.03E-34))
    );
    assert_eq!(
        double_lit(Span::new("1.03e+34LF")),
        Ok((Span::new(""), 1.03e+34))
    );
    assert_eq!(
        double_lit(Span::new("1.03E+34LF")),
        Ok((Span::new(""), 1.03E+34))
    );
    assert_eq!(
        double_lit(Span::new("1.03e-34LF")),
        Ok((Span::new(""), 1.03e-34))
    );
    assert_eq!(
        double_lit(Span::new("1.03E-34LF")),
        Ok((Span::new(""), 1.03E-34))
    );
}

#[test]
fn parse_double_neg_lit() {
    assert_eq!(double_lit(Span::new("-0.;")), Ok((Span::new(";"), -0.)));
    assert_eq!(double_lit(Span::new("-.0;")), Ok((Span::new(";"), -0.)));
    assert_eq!(
        double_lit(Span::new("-.035 ")),
        Ok((Span::new(" "), -0.035))
    );
    assert_eq!(double_lit(Span::new("-0. ")), Ok((Span::new(" "), -0.)));
    assert_eq!(
        double_lit(Span::new("-0.035 ")),
        Ok((Span::new(" "), -0.035))
    );
    assert_eq!(double_lit(Span::new("-0.lf")), Ok((Span::new(""), -0.)));
    assert_eq!(
        double_lit(Span::new("-0.035lf")),
        Ok((Span::new(""), -0.035))
    );
    assert_eq!(
        double_lit(Span::new("-.035lf")),
        Ok((Span::new(""), -0.035))
    );
    assert_eq!(
        double_lit(Span::new("-.035LF")),
        Ok((Span::new(""), -0.035))
    );
    assert_eq!(double_lit(Span::new("-0.LF")), Ok((Span::new(""), -0.)));
    assert_eq!(
        double_lit(Span::new("-0.035LF")),
        Ok((Span::new(""), -0.035))
    );
    assert_eq!(
        double_lit(Span::new("-1.03e+34lf")),
        Ok((Span::new(""), -1.03e+34))
    );
    assert_eq!(
        double_lit(Span::new("-1.03E+34lf")),
        Ok((Span::new(""), -1.03E+34))
    );
    assert_eq!(
        double_lit(Span::new("-1.03e-34lf")),
        Ok((Span::new(""), -1.03e-34))
    );
    assert_eq!(
        double_lit(Span::new("-1.03E-34lf")),
        Ok((Span::new(""), -1.03E-34))
    );
    assert_eq!(
        double_lit(Span::new("-1.03e+34LF")),
        Ok((Span::new(""), -1.03e+34))
    );
    assert_eq!(
        double_lit(Span::new("-1.03E+34LF")),
        Ok((Span::new(""), -1.03E+34))
    );
    assert_eq!(
        double_lit(Span::new("-1.03e-34LF")),
        Ok((Span::new(""), -1.03e-34))
    );
    assert_eq!(
        double_lit(Span::new("-1.03E-34LF")),
        Ok((Span::new(""), -1.03E-34))
    );
}

#[test]
fn parse_bool_lit() {
    assert_eq!(bool_lit(Span::new("false")), Ok((Span::new(""), false)));
    assert_eq!(bool_lit(Span::new("true")), Ok((Span::new(""), true)));
}

#[test]
fn parse_identifier() {
    assert_eq!(
        identifier(Span::new("a")),
        Ok((Span::new(""), ("a").into()))
    );
    assert_eq!(
        identifier(Span::new("ab_cd")),
        Ok((Span::new(""), ("ab_cd").into()))
    );
    assert_eq!(
        identifier(Span::new("Ab_cd")),
        Ok((Span::new(""), ("Ab_cd").into()))
    );
    assert_eq!(
        identifier(Span::new("Ab_c8d")),
        Ok((Span::new(""), ("Ab_c8d").into()))
    );
    assert_eq!(
        identifier(Span::new("Ab_c8d9")),
        Ok((Span::new(""), ("Ab_c8d9").into()))
    );
}

#[test]
fn parse_unary_op_add() {
    assert_eq!(
        unary_op(Span::new("+ ")),
        Ok((Span::new(" "), syntax::UnaryOp::Add))
    );
}

#[test]
fn parse_unary_op_minus() {
    assert_eq!(
        unary_op(Span::new("- ")),
        Ok((Span::new(" "), syntax::UnaryOp::Minus))
    );
}

#[test]
fn parse_unary_op_not() {
    assert_eq!(
        unary_op(Span::new("!")),
        Ok((Span::new(""), syntax::UnaryOp::Not))
    );
}

#[test]
fn parse_unary_op_complement() {
    assert_eq!(
        unary_op(Span::new("~")),
        Ok((Span::new(""), syntax::UnaryOp::Complement))
    );
}

#[test]
fn parse_unary_op_inc() {
    assert_eq!(
        unary_op(Span::new("++")),
        Ok((Span::new(""), syntax::UnaryOp::Inc))
    );
}

#[test]
fn parse_unary_op_dec() {
    assert_eq!(
        unary_op(Span::new("--")),
        Ok((Span::new(""), syntax::UnaryOp::Dec))
    );
}

#[test]
fn parse_array_specifier_dimension_unsized() {
    assert_eq!(
        array_specifier_dimension(Span::new("[]")),
        Ok((Span::new(""), syntax::ArraySpecifierDimension::Unsized))
    );
    assert_eq!(
        array_specifier_dimension(Span::new("[ ]")),
        Ok((Span::new(""), syntax::ArraySpecifierDimension::Unsized))
    );
    assert_eq!(
        array_specifier_dimension(Span::new("[\n]")),
        Ok((Span::new(""), syntax::ArraySpecifierDimension::Unsized))
    );
}

#[test]
fn parse_array_specifier_dimension_sized() {
    let ix = syntax::Expr::IntConst(0);

    assert_eq!(
        array_specifier_dimension(Span::new("[0]")),
        Ok((
            Span::new(""),
            syntax::ArraySpecifierDimension::ExplicitlySized(Box::new(ix.clone()))
        ))
    );
    assert_eq!(
        array_specifier_dimension(Span::new("[\n0   \t]")),
        Ok((
            Span::new(""),
            syntax::ArraySpecifierDimension::ExplicitlySized(Box::new(ix))
        ))
    );
}

#[test]
fn parse_array_specifier_unsized() {
    assert_eq!(
        array_specifier(Span::new("[]")),
        Ok((
            Span::new(""),
            syntax::ArraySpecifier {
                dimensions: syntax::NonEmpty(vec![syntax::ArraySpecifierDimension::Unsized])
            }
        ))
    )
}

#[test]
fn parse_array_specifier_sized() {
    let ix = syntax::Expr::IntConst(123);

    assert_eq!(
        array_specifier(Span::new("[123]")),
        Ok((
            Span::new(""),
            syntax::ArraySpecifier {
                dimensions: syntax::NonEmpty(vec![
                    syntax::ArraySpecifierDimension::ExplicitlySized(Box::new(ix))
                ])
            }
        ))
    )
}

#[test]
fn parse_array_specifier_sized_multiple() {
    let a = syntax::Expr::IntConst(2);
    let b = syntax::Expr::IntConst(100);
    let d = syntax::Expr::IntConst(5);

    assert_eq!(
        array_specifier(Span::new("[2][100][][5]")),
        Ok((
            Span::new(""),
            syntax::ArraySpecifier {
                dimensions: syntax::NonEmpty(vec![
                    syntax::ArraySpecifierDimension::ExplicitlySized(Box::new(a)),
                    syntax::ArraySpecifierDimension::ExplicitlySized(Box::new(b)),
                    syntax::ArraySpecifierDimension::Unsized,
                    syntax::ArraySpecifierDimension::ExplicitlySized(Box::new(d)),
                ])
            }
        ))
    )
}

#[test]
fn parse_precise_qualifier() {
    assert_eq!(
        precise_qualifier(Span::new("precise ")),
        Ok((Span::new(" "), ()))
    );
}

#[test]
fn parse_invariant_qualifier() {
    assert_eq!(
        invariant_qualifier(Span::new("invariant ")),
        Ok((Span::new(" "), ()))
    );
}

#[test]
fn parse_interpolation_qualifier() {
    assert_eq!(
        interpolation_qualifier(Span::new("smooth ")),
        Ok((Span::new(" "), syntax::InterpolationQualifier::Smooth))
    );
    assert_eq!(
        interpolation_qualifier(Span::new("flat ")),
        Ok((Span::new(" "), syntax::InterpolationQualifier::Flat))
    );
    assert_eq!(
        interpolation_qualifier(Span::new("noperspective ")),
        Ok((
            Span::new(" "),
            syntax::InterpolationQualifier::NoPerspective
        ))
    );
}

#[test]
fn parse_precision_qualifier() {
    assert_eq!(
        precision_qualifier(Span::new("highp ")),
        Ok((Span::new(" "), syntax::PrecisionQualifier::High))
    );
    assert_eq!(
        precision_qualifier(Span::new("mediump ")),
        Ok((Span::new(" "), syntax::PrecisionQualifier::Medium))
    );
    assert_eq!(
        precision_qualifier(Span::new("lowp ")),
        Ok((Span::new(" "), syntax::PrecisionQualifier::Low))
    );
}

#[test]
fn parse_storage_qualifier() {
    assert_eq!(
        storage_qualifier(Span::new("const ")),
        Ok((Span::new(" "), syntax::StorageQualifier::Const))
    );
    assert_eq!(
        storage_qualifier(Span::new("inout ")),
        Ok((Span::new(" "), syntax::StorageQualifier::InOut))
    );
    assert_eq!(
        storage_qualifier(Span::new("in ")),
        Ok((Span::new(" "), syntax::StorageQualifier::In))
    );
    assert_eq!(
        storage_qualifier(Span::new("out ")),
        Ok((Span::new(" "), syntax::StorageQualifier::Out))
    );
    assert_eq!(
        storage_qualifier(Span::new("centroid ")),
        Ok((Span::new(" "), syntax::StorageQualifier::Centroid))
    );
    assert_eq!(
        storage_qualifier(Span::new("patch ")),
        Ok((Span::new(" "), syntax::StorageQualifier::Patch))
    );
    assert_eq!(
        storage_qualifier(Span::new("sample ")),
        Ok((Span::new(" "), syntax::StorageQualifier::Sample))
    );
    assert_eq!(
        storage_qualifier(Span::new("uniform ")),
        Ok((Span::new(" "), syntax::StorageQualifier::Uniform))
    );
    assert_eq!(
        storage_qualifier(Span::new("attribute ")),
        Ok((Span::new(" "), syntax::StorageQualifier::Attribute))
    );
    assert_eq!(
        storage_qualifier(Span::new("varying ")),
        Ok((Span::new(" "), syntax::StorageQualifier::Varying))
    );
    assert_eq!(
        storage_qualifier(Span::new("buffer ")),
        Ok((Span::new(" "), syntax::StorageQualifier::Buffer))
    );
    assert_eq!(
        storage_qualifier(Span::new("shared ")),
        Ok((Span::new(" "), syntax::StorageQualifier::Shared))
    );
    assert_eq!(
        storage_qualifier(Span::new("coherent ")),
        Ok((Span::new(" "), syntax::StorageQualifier::Coherent))
    );
    assert_eq!(
        storage_qualifier(Span::new("volatile ")),
        Ok((Span::new(" "), syntax::StorageQualifier::Volatile))
    );
    assert_eq!(
        storage_qualifier(Span::new("restrict ")),
        Ok((Span::new(" "), syntax::StorageQualifier::Restrict))
    );
    assert_eq!(
        storage_qualifier(Span::new("readonly ")),
        Ok((Span::new(" "), syntax::StorageQualifier::ReadOnly))
    );
    assert_eq!(
        storage_qualifier(Span::new("writeonly ")),
        Ok((Span::new(" "), syntax::StorageQualifier::WriteOnly))
    );
    assert_eq!(
        storage_qualifier(Span::new("subroutine a")),
        Ok((
            Span::new(" a"),
            syntax::StorageQualifier::Subroutine(vec![])
        ))
    );

    let a = syntax::TypeName("vec3".to_owned());
    let b = syntax::TypeName(("float").to_owned());
    let c = syntax::TypeName(("dmat43").to_owned());
    let types = vec![a, b, c];
    assert_eq!(
        storage_qualifier(Span::new("subroutine (  vec3 , float \\\n, dmat43)")),
        Ok((Span::new(""), syntax::StorageQualifier::Subroutine(types)))
    );
}

#[test]
fn parse_layout_qualifier_std430() {
    let expected = syntax::LayoutQualifier {
        ids: syntax::NonEmpty(vec![syntax::LayoutQualifierSpec::Identifier(
            ("std430").into(),
            None,
        )]),
    };

    assert_eq!(
        layout_qualifier(Span::new("layout (std430)")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        layout_qualifier(Span::new("layout  (std430   )")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        layout_qualifier(Span::new("layout \n\t (  std430  )")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        layout_qualifier(Span::new("layout(std430)")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_layout_qualifier_shared() {
    let expected = syntax::LayoutQualifier {
        ids: syntax::NonEmpty(vec![syntax::LayoutQualifierSpec::Shared]),
    };

    assert_eq!(
        layout_qualifier(Span::new("layout (shared)")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        layout_qualifier(Span::new("layout ( shared )")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        layout_qualifier(Span::new("layout(shared)")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_layout_qualifier_list() {
    let id_0 = syntax::LayoutQualifierSpec::Shared;
    let id_1 = syntax::LayoutQualifierSpec::Identifier(("std140").into(), None);
    let id_2 = syntax::LayoutQualifierSpec::Identifier(
        ("max_vertices").into(),
        Some(Box::new(syntax::Expr::IntConst(3))),
    );
    let expected = syntax::LayoutQualifier {
        ids: syntax::NonEmpty(vec![id_0, id_1, id_2]),
    };

    assert_eq!(
        layout_qualifier(Span::new("layout (shared, std140, max_vertices = 3)")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        layout_qualifier(Span::new("layout(shared,std140,max_vertices=3)")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        layout_qualifier(Span::new(
            "layout\n\n\t (    shared , std140, max_vertices= 3)"
        )),
        Ok((Span::new(""), expected.clone()))
    );
}

#[test]
fn parse_type_qualifier() {
    let storage_qual = syntax::TypeQualifierSpec::Storage(syntax::StorageQualifier::Const);
    let id_0 = syntax::LayoutQualifierSpec::Shared;
    let id_1 = syntax::LayoutQualifierSpec::Identifier(("std140").into(), None);
    let id_2 = syntax::LayoutQualifierSpec::Identifier(
        ("max_vertices").into(),
        Some(Box::new(syntax::Expr::IntConst(3))),
    );
    let layout_qual = syntax::TypeQualifierSpec::Layout(syntax::LayoutQualifier {
        ids: syntax::NonEmpty(vec![id_0, id_1, id_2]),
    });
    let expected = syntax::TypeQualifier {
        qualifiers: syntax::NonEmpty(vec![storage_qual, layout_qual]),
    };

    assert_eq!(
        type_qualifier(Span::new("const layout (shared, std140, max_vertices = 3)")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        type_qualifier(Span::new("const layout(shared,std140,max_vertices=3)")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_struct_field_specifier() {
    let expected = syntax::StructFieldSpecifier {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::Vec4,
            array_specifier: None,
        },
        identifiers: syntax::NonEmpty(vec![("foo").into()]),
    };

    assert_eq!(
        struct_field_specifier(Span::new("vec4 foo;")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        struct_field_specifier(Span::new("vec4     foo ; ")),
        Ok((Span::new(" "), expected.clone()))
    );
}

#[test]
fn parse_struct_field_specifier_type_name() {
    let expected = syntax::StructFieldSpecifier {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::TypeName(("S0238_3").into()),
            array_specifier: None,
        },
        identifiers: syntax::NonEmpty(vec![("x").into()]),
    };

    assert_eq!(
        struct_field_specifier(Span::new("S0238_3 x;")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        struct_field_specifier(Span::new("S0238_3     x ;")),
        Ok((Span::new(""), expected.clone()))
    );
}

#[test]
fn parse_struct_field_specifier_several() {
    let expected = syntax::StructFieldSpecifier {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::Vec4,
            array_specifier: None,
        },
        identifiers: syntax::NonEmpty(vec![("foo").into(), ("bar").into(), ("zoo").into()]),
    };

    assert_eq!(
        struct_field_specifier(Span::new("vec4 foo, bar, zoo;")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        struct_field_specifier(Span::new("vec4     foo , bar  , zoo ;")),
        Ok((Span::new(""), expected.clone()))
    );
}

#[test]
fn parse_struct_specifier_one_field() {
    let field = syntax::StructFieldSpecifier {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::Vec4,
            array_specifier: None,
        },
        identifiers: syntax::NonEmpty(vec![("foo").into()]),
    };
    let expected = syntax::StructSpecifier {
        name: Some(("TestStruct").into()),
        fields: syntax::NonEmpty(vec![field]),
    };

    assert_eq!(
        struct_specifier(Span::new("struct TestStruct { vec4 foo; }")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        struct_specifier(Span::new(
            "struct      TestStruct \n \n\n {\n    vec4   foo  ;}"
        )),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_struct_specifier_multi_fields() {
    let a = syntax::StructFieldSpecifier {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::Vec4,
            array_specifier: None,
        },
        identifiers: syntax::NonEmpty(vec![("foo").into()]),
    };
    let b = syntax::StructFieldSpecifier {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::Float,
            array_specifier: None,
        },
        identifiers: syntax::NonEmpty(vec![("bar").into()]),
    };
    let c = syntax::StructFieldSpecifier {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::UInt,
            array_specifier: None,
        },
        identifiers: syntax::NonEmpty(vec![("zoo").into()]),
    };
    let d = syntax::StructFieldSpecifier {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::BVec3,
            array_specifier: None,
        },
        identifiers: syntax::NonEmpty(vec![("foo_BAR_zoo3497_34").into()]),
    };
    let e = syntax::StructFieldSpecifier {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::TypeName(("S0238_3").into()),
            array_specifier: None,
        },
        identifiers: syntax::NonEmpty(vec![("x").into()]),
    };
    let expected = syntax::StructSpecifier {
        name: Some(("_TestStruct_934i").into()),
        fields: syntax::NonEmpty(vec![a, b, c, d, e]),
    };

    assert_eq!(
    struct_specifier(
      Span::new("struct _TestStruct_934i { vec4 foo; float bar; uint zoo; bvec3 foo_BAR_zoo3497_34; S0238_3 x; }")
    ),
    Ok((Span::new(""), expected.clone()))
  );
    assert_eq!(
    struct_specifier(
      Span::new("struct _TestStruct_934i{vec4 foo;float bar;uint zoo;bvec3 foo_BAR_zoo3497_34;S0238_3 x;}")
    ),
    Ok((Span::new(""), expected.clone()))
  );
    assert_eq!(struct_specifier(Span::new("struct _TestStruct_934i\n   {  vec4\nfoo ;   \n\t float\n\t\t  bar  ;   \nuint   zoo;    \n bvec3   foo_BAR_zoo3497_34\n\n\t\n\t\n  ; S0238_3 x;}")), Ok((Span::new(""), expected)));
}

#[test]
fn parse_type_specifier_non_array() {
    assert_eq!(
        type_specifier_non_array(Span::new("bool")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Bool))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("int")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Int))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("uint")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::UInt))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("float")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Float))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("double")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Double))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("vec2")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Vec2))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("vec3")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Vec3))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("vec4")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Vec4))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("dvec2")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::DVec2))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("dvec3")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::DVec3))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("dvec4")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::DVec4))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("bvec2")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::BVec2))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("bvec3")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::BVec3))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("bvec4")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::BVec4))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("ivec2")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::IVec2))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("ivec3")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::IVec3))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("ivec4")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::IVec4))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("uvec2")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::UVec2))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("uvec3")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::UVec3))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("uvec4")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::UVec4))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("mat2")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Mat2))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("mat3")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Mat3))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("mat4")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Mat4))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("mat2x2")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Mat2))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("mat2x3")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Mat23))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("mat2x4")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Mat24))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("mat3x2")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Mat32))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("mat3x3")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Mat3))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("mat3x4")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Mat34))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("mat4x2")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Mat42))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("mat4x3")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Mat43))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("mat4x4")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Mat4))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("dmat2")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::DMat2))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("dmat3")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::DMat3))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("dmat4")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::DMat4))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("dmat2x2")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::DMat2))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("dmat2x3")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::DMat23))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("dmat2x4")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::DMat24))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("dmat3x2")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::DMat32))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("dmat3x3")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::DMat3))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("dmat3x4")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::DMat34))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("dmat4x2")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::DMat42))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("dmat4x3")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::DMat43))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("dmat4x4")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::DMat4))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("sampler1D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Sampler1D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("image1D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Image1D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("sampler2D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Sampler2D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("image2D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Image2D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("sampler3D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Sampler3D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("image3D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Image3D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("samplerCube")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::SamplerCube))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("imageCube")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::ImageCube))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("sampler2DRect")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Sampler2DRect))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("image2DRect")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Image2DRect))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("sampler1DArray")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Sampler1DArray))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("image1DArray")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Image1DArray))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("sampler2DArray")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Sampler2DArray))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("image2DArray")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Image2DArray))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("samplerBuffer")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::SamplerBuffer))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("imageBuffer")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::ImageBuffer))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("sampler2DMS")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Sampler2DMS))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("image2DMS")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Image2DMS))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("sampler2DMSArray")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::Sampler2DMSArray
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("image2DMSArray")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::Image2DMSArray))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("samplerCubeArray")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::SamplerCubeArray
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("imageCubeArray")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::ImageCubeArray))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("sampler1DShadow")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::Sampler1DShadow
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("sampler2DShadow")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::Sampler2DShadow
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("sampler2DRectShadow")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::Sampler2DRectShadow
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("sampler1DArrayShadow")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::Sampler1DArrayShadow
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("sampler2DArrayShadow")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::Sampler2DArrayShadow
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("samplerCubeShadow")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::SamplerCubeShadow
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("samplerCubeArrayShadow")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::SamplerCubeArrayShadow
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("isampler1D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::ISampler1D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("iimage1D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::IImage1D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("isampler2D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::ISampler2D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("iimage2D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::IImage2D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("isampler3D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::ISampler3D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("iimage3D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::IImage3D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("isamplerCube")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::ISamplerCube))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("iimageCube")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::IImageCube))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("isampler2DRect")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::ISampler2DRect))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("iimage2DRect")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::IImage2DRect))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("isampler1DArray")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::ISampler1DArray
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("iimage1DArray")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::IImage1DArray))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("isampler2DArray")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::ISampler2DArray
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("iimage2DArray")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::IImage2DArray))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("isamplerBuffer")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::ISamplerBuffer))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("iimageBuffer")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::IImageBuffer))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("isampler2DMS")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::ISampler2DMS))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("iimage2DMS")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::IImage2DMS))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("isampler2DMSArray")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::ISampler2DMSArray
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("iimage2DMSArray")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::IImage2DMSArray
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("isamplerCubeArray")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::ISamplerCubeArray
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("iimageCubeArray")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::IImageCubeArray
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("atomic_uint")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::AtomicUInt))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("usampler1D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::USampler1D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("uimage1D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::UImage1D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("usampler2D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::USampler2D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("uimage2D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::UImage2D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("usampler3D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::USampler3D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("uimage3D")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::UImage3D))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("usamplerCube")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::USamplerCube))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("uimageCube")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::UImageCube))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("usampler2DRect")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::USampler2DRect))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("uimage2DRect")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::UImage2DRect))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("usampler1DArray")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::USampler1DArray
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("uimage1DArray")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::UImage1DArray))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("usampler2DArray")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::USampler2DArray
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("uimage2DArray")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::UImage2DArray))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("usamplerBuffer")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::USamplerBuffer))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("uimageBuffer")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::UImageBuffer))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("usampler2DMS")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::USampler2DMS))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("uimage2DMS")),
        Ok((Span::new(""), syntax::TypeSpecifierNonArray::UImage2DMS))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("usampler2DMSArray")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::USampler2DMSArray
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("uimage2DMSArray")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::UImage2DMSArray
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("usamplerCubeArray")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::USamplerCubeArray
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("uimageCubeArray")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::UImageCubeArray
        ))
    );
    assert_eq!(
        type_specifier_non_array(Span::new("ReturnType")),
        Ok((
            Span::new(""),
            syntax::TypeSpecifierNonArray::TypeName(syntax::TypeName::new(("ReturnType")).unwrap())
        ))
    );
}

#[test]
fn parse_type_specifier() {
    assert_eq!(
        type_specifier(Span::new("uint;")),
        Ok((
            Span::new(";"),
            syntax::TypeSpecifier {
                ty: syntax::TypeSpecifierNonArray::UInt,
                array_specifier: None
            }
        ))
    );
    assert_eq!(
        type_specifier(Span::new("iimage2DMSArray[35];")),
        Ok((
            Span::new(";"),
            syntax::TypeSpecifier {
                ty: syntax::TypeSpecifierNonArray::IImage2DMSArray,
                array_specifier: Some(syntax::ArraySpecifier {
                    dimensions: syntax::NonEmpty(vec![
                        syntax::ArraySpecifierDimension::ExplicitlySized(Box::new(
                            syntax::Expr::IntConst(35)
                        ))
                    ])
                })
            }
        ))
    );
}

#[test]
fn parse_fully_specified_type() {
    let ty = syntax::TypeSpecifier {
        ty: syntax::TypeSpecifierNonArray::IImage2DMSArray,
        array_specifier: None,
    };
    let expected = syntax::FullySpecifiedType {
        qualifier: None,
        ty,
    };

    assert_eq!(
        fully_specified_type(Span::new("iimage2DMSArray;")),
        Ok((Span::new(";"), expected.clone()))
    );
}

#[test]
fn parse_fully_specified_type_with_qualifier() {
    let qual_spec = syntax::TypeQualifierSpec::Storage(syntax::StorageQualifier::Subroutine(vec![
        ("vec2").into(),
        ("S032_29k").into(),
    ]));
    let qual = syntax::TypeQualifier {
        qualifiers: syntax::NonEmpty(vec![qual_spec]),
    };
    let ty = syntax::TypeSpecifier {
        ty: syntax::TypeSpecifierNonArray::IImage2DMSArray,
        array_specifier: None,
    };
    let expected = syntax::FullySpecifiedType {
        qualifier: Some(qual),
        ty,
    };

    assert_eq!(
        fully_specified_type(Span::new("subroutine (vec2, S032_29k) iimage2DMSArray;")),
        Ok((Span::new(";"), expected.clone()))
    );
    assert_eq!(
        fully_specified_type(Span::new(
            "subroutine (  vec2\t\n \t , \n S032_29k   )\n iimage2DMSArray ;"
        )),
        Ok((Span::new(" ;"), expected.clone()))
    );
    assert_eq!(
        fully_specified_type(Span::new("subroutine(vec2,S032_29k)iimage2DMSArray;")),
        Ok((Span::new(";"), expected))
    );
}

#[test]
fn parse_primary_expr_intconst() {
    assert_eq!(
        primary_expr(Span::new("0 ")),
        Ok((Span::new(" "), syntax::Expr::IntConst(0)))
    );
    assert_eq!(
        primary_expr(Span::new("1 ")),
        Ok((Span::new(" "), syntax::Expr::IntConst(1)))
    );
}

#[test]
fn parse_primary_expr_uintconst() {
    assert_eq!(
        primary_expr(Span::new("0u ")),
        Ok((Span::new(" "), syntax::Expr::UIntConst(0)))
    );
    assert_eq!(
        primary_expr(Span::new("1u ")),
        Ok((Span::new(" "), syntax::Expr::UIntConst(1)))
    );
}

#[test]
fn parse_primary_expr_floatconst() {
    assert_eq!(
        primary_expr(Span::new("0.f ")),
        Ok((Span::new(" "), syntax::Expr::FloatConst(0.)))
    );
    assert_eq!(
        primary_expr(Span::new("1.f ")),
        Ok((Span::new(" "), syntax::Expr::FloatConst(1.)))
    );
    assert_eq!(
        primary_expr(Span::new("0.F ")),
        Ok((Span::new(" "), syntax::Expr::FloatConst(0.)))
    );
    assert_eq!(
        primary_expr(Span::new("1.F ")),
        Ok((Span::new(" "), syntax::Expr::FloatConst(1.)))
    );
}

#[test]
fn parse_primary_expr_doubleconst() {
    assert_eq!(
        primary_expr(Span::new("0. ")),
        Ok((Span::new(" "), syntax::Expr::FloatConst(0.)))
    );
    assert_eq!(
        primary_expr(Span::new("1. ")),
        Ok((Span::new(" "), syntax::Expr::FloatConst(1.)))
    );
    assert_eq!(
        primary_expr(Span::new("0.lf ")),
        Ok((Span::new(" "), syntax::Expr::DoubleConst(0.)))
    );
    assert_eq!(
        primary_expr(Span::new("1.lf ")),
        Ok((Span::new(" "), syntax::Expr::DoubleConst(1.)))
    );
    assert_eq!(
        primary_expr(Span::new("0.LF ")),
        Ok((Span::new(" "), syntax::Expr::DoubleConst(0.)))
    );
    assert_eq!(
        primary_expr(Span::new("1.LF ")),
        Ok((Span::new(" "), syntax::Expr::DoubleConst(1.)))
    );
}

#[test]
fn parse_primary_expr_boolconst() {
    assert_eq!(
        primary_expr(Span::new("false")),
        Ok((Span::new(""), syntax::Expr::BoolConst(false.to_owned())))
    );
    assert_eq!(
        primary_expr(Span::new("true")),
        Ok((Span::new(""), syntax::Expr::BoolConst(true.to_owned())))
    );
}

#[test]
fn parse_primary_expr_parens() {
    assert_eq!(
        primary_expr(Span::new("(0)")),
        Ok((Span::new(""), syntax::Expr::IntConst(0)))
    );
    assert_eq!(
        primary_expr(Span::new("(  0 )")),
        Ok((Span::new(""), syntax::Expr::IntConst(0)))
    );
    assert_eq!(
        primary_expr(Span::new("(  .0 )")),
        Ok((Span::new(""), syntax::Expr::FloatConst(0.)))
    );
    assert_eq!(
        primary_expr(Span::new("(  (.0) )")),
        Ok((Span::new(""), syntax::Expr::FloatConst(0.)))
    );
    assert_eq!(
        primary_expr(Span::new("(true) ")),
        Ok((Span::new(" "), syntax::Expr::BoolConst(true)))
    );
}

#[test]
fn parse_postfix_function_call_no_args() {
    let fun = syntax::FunIdentifier::Identifier(("vec3").into());
    let args = Vec::new();
    let expected = syntax::Expr::FunCall(fun, args);

    assert_eq!(
        postfix_expr(Span::new("vec3();")),
        Ok((Span::new(";"), expected.clone()))
    );
    assert_eq!(
        postfix_expr(Span::new("vec3   (  ) ;")),
        Ok((Span::new(" ;"), expected.clone()))
    );
    assert_eq!(
        postfix_expr(Span::new("vec3   (\nvoid\n) ;")),
        Ok((Span::new(" ;"), expected))
    );
}

#[test]
fn parse_postfix_function_call_one_arg() {
    let fun = syntax::FunIdentifier::Identifier(("foo").into());
    let args = vec![syntax::Expr::IntConst(0)];
    let expected = syntax::Expr::FunCall(fun, args);

    assert_eq!(
        postfix_expr(Span::new("foo(0);")),
        Ok((Span::new(";"), expected.clone()))
    );
    assert_eq!(
        postfix_expr(Span::new("foo   ( 0 ) ;")),
        Ok((Span::new(" ;"), expected.clone()))
    );
    assert_eq!(
        postfix_expr(Span::new("foo   (\n0\t\n) ;")),
        Ok((Span::new(" ;"), expected))
    );
}

#[test]
fn parse_postfix_function_call_multi_arg() {
    let fun = syntax::FunIdentifier::Identifier(("foo").into());
    let args = vec![
        syntax::Expr::IntConst(0),
        syntax::Expr::BoolConst(false),
        syntax::Expr::Variable(("bar").into()),
    ];
    let expected = syntax::Expr::FunCall(fun, args);

    assert_eq!(
        postfix_expr(Span::new("foo(0, false, bar);")),
        Ok((Span::new(";"), expected.clone()))
    );
    assert_eq!(
        postfix_expr(Span::new("foo   ( 0\t, false    ,\t\tbar) ;")),
        Ok((Span::new(" ;"), expected))
    );
}

#[test]
fn parse_postfix_expr_bracket() {
    let id = syntax::Expr::Variable(("foo").into());
    let array_spec = syntax::ArraySpecifier {
        dimensions: syntax::NonEmpty(vec![syntax::ArraySpecifierDimension::ExplicitlySized(
            Box::new(syntax::Expr::IntConst(7354)),
        )]),
    };
    let expected = syntax::Expr::Bracket(Box::new(id), array_spec);

    assert_eq!(
        postfix_expr(Span::new("foo[7354];")),
        Ok((Span::new(";"), expected.clone()))
    );
    assert_eq!(
        postfix_expr(Span::new("foo[\n  7354    ] ;")),
        Ok((Span::new(";"), expected))
    );
}

#[test]
fn parse_postfix_expr_dot() {
    let foo = Box::new(syntax::Expr::Variable(("foo").into()));
    let expected = syntax::Expr::Dot(foo, ("bar").into());

    assert_eq!(
        postfix_expr(Span::new("foo.bar;")),
        Ok((Span::new(";"), expected.clone()))
    );
    assert_eq!(
        postfix_expr(Span::new("(foo).bar;")),
        Ok((Span::new(";"), expected))
    );
}

#[test]
fn parse_postfix_expr_dot_several() {
    let foo = Box::new(syntax::Expr::Variable(("foo").into()));
    let expected = syntax::Expr::Dot(
        Box::new(syntax::Expr::Dot(foo, ("bar").into())),
        ("zoo").into(),
    );

    assert_eq!(
        postfix_expr(Span::new("foo.bar.zoo;")),
        Ok((Span::new(";"), expected.clone()))
    );
    assert_eq!(
        postfix_expr(Span::new("(foo).bar.zoo;")),
        Ok((Span::new(";"), expected.clone()))
    );
    assert_eq!(
        postfix_expr(Span::new("(foo.bar).zoo;")),
        Ok((Span::new(";"), expected))
    );
}

#[test]
fn parse_postfix_postinc() {
    let foo = syntax::Expr::Variable(("foo").into());
    let expected = syntax::Expr::PostInc(Box::new(foo));

    assert_eq!(
        postfix_expr(Span::new("foo++;")),
        Ok((Span::new(";"), expected.clone()))
    );
}

#[test]
fn parse_postfix_postdec() {
    let foo = syntax::Expr::Variable(("foo").into());
    let expected = syntax::Expr::PostDec(Box::new(foo));

    assert_eq!(
        postfix_expr(Span::new("foo--;")),
        Ok((Span::new(";"), expected.clone()))
    );
}

#[test]
fn parse_unary_add() {
    let foo = syntax::Expr::Variable(("foo").into());
    let expected = syntax::Expr::Unary(syntax::UnaryOp::Add, Box::new(foo));

    assert_eq!(
        unary_expr(Span::new("+foo;")),
        Ok((Span::new(";"), expected.clone()))
    );
}

#[test]
fn parse_unary_minus() {
    let foo = syntax::Expr::Variable(("foo").into());
    let expected = syntax::Expr::Unary(syntax::UnaryOp::Minus, Box::new(foo));

    assert_eq!(
        unary_expr(Span::new("-foo;")),
        Ok((Span::new(";"), expected.clone()))
    );
}

#[test]
fn parse_unary_not() {
    let foo = syntax::Expr::Variable(("foo").into());
    let expected = syntax::Expr::Unary(syntax::UnaryOp::Not, Box::new(foo));

    assert_eq!(
        unary_expr(Span::new("!foo;")),
        Ok((Span::new(";"), expected))
    );
}

#[test]
fn parse_unary_complement() {
    let foo = syntax::Expr::Variable(("foo").into());
    let expected = syntax::Expr::Unary(syntax::UnaryOp::Complement, Box::new(foo));

    assert_eq!(
        unary_expr(Span::new("~foo;")),
        Ok((Span::new(";"), expected.clone()))
    );
}

#[test]
fn parse_unary_inc() {
    let foo = syntax::Expr::Variable(("foo").into());
    let expected = syntax::Expr::Unary(syntax::UnaryOp::Inc, Box::new(foo));

    assert_eq!(
        unary_expr(Span::new("++foo;")),
        Ok((Span::new(";"), expected.clone()))
    );
}

#[test]
fn parse_unary_dec() {
    let foo = syntax::Expr::Variable(("foo").into());
    let expected = syntax::Expr::Unary(syntax::UnaryOp::Dec, Box::new(foo));

    assert_eq!(
        unary_expr(Span::new("--foo;")),
        Ok((Span::new(";"), expected.clone()))
    );
}

#[test]
fn parse_expr_float() {
    assert_eq!(
        expr(Span::new("314.;")),
        Ok((Span::new(";"), syntax::Expr::FloatConst(314.)))
    );
    assert_eq!(
        expr(Span::new("314.f;")),
        Ok((Span::new(";"), syntax::Expr::FloatConst(314.)))
    );
    assert_eq!(
        expr(Span::new("314.LF;")),
        Ok((Span::new(";"), syntax::Expr::DoubleConst(314.)))
    );
}

#[test]
fn parse_expr_add_2() {
    let one = Box::new(syntax::Expr::IntConst(1));
    let expected = syntax::Expr::Binary(syntax::BinaryOp::Add, one.clone(), one);

    assert_eq!(
        expr(Span::new("1 + 1;")),
        Ok((Span::new(";"), expected.clone()))
    );
    assert_eq!(
        expr(Span::new("1+1;")),
        Ok((Span::new(";"), expected.clone()))
    );
    assert_eq!(expr(Span::new("(1 + 1);")), Ok((Span::new(";"), expected)));
}

#[test]
fn parse_expr_add_3() {
    let one = Box::new(syntax::Expr::UIntConst(1));
    let two = Box::new(syntax::Expr::UIntConst(2));
    let three = Box::new(syntax::Expr::UIntConst(3));
    let expected = syntax::Expr::Binary(
        syntax::BinaryOp::Add,
        Box::new(syntax::Expr::Binary(syntax::BinaryOp::Add, one, two)),
        three,
    );

    assert_eq!(
        expr(Span::new("1u + 2u + 3u")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        expr(Span::new("1u + 2u + 3u   ")),
        Ok((Span::new("   "), expected.clone()))
    );
    assert_eq!(
        expr(Span::new("1u+2u+3u")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        expr(Span::new("((1u + 2u) + 3u)")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_expr_add_mult_3() {
    let one = Box::new(syntax::Expr::UIntConst(1));
    let two = Box::new(syntax::Expr::UIntConst(2));
    let three = Box::new(syntax::Expr::UIntConst(3));
    let expected = syntax::Expr::Binary(
        syntax::BinaryOp::Add,
        Box::new(syntax::Expr::Binary(syntax::BinaryOp::Mult, one, two)),
        three,
    );

    assert_eq!(
        expr(Span::new("1u * 2u + 3u ;")),
        Ok((Span::new(" ;"), expected.clone()))
    );
    assert_eq!(
        expr(Span::new("1u*2u+3u;")),
        Ok((Span::new(";"), expected.clone()))
    );
    assert_eq!(
        expr(Span::new("(1u * 2u) + 3u;")),
        Ok((Span::new(";"), expected))
    );
}

#[test]
fn parse_expr_add_sub_mult_div() {
    let one = Box::new(syntax::Expr::IntConst(1));
    let two = Box::new(syntax::Expr::IntConst(2));
    let three = Box::new(syntax::Expr::IntConst(3));
    let four = Box::new(syntax::Expr::IntConst(4));
    let five = Box::new(syntax::Expr::IntConst(5));
    let six = Box::new(syntax::Expr::IntConst(6));
    let expected = syntax::Expr::Binary(
        syntax::BinaryOp::Add,
        Box::new(syntax::Expr::Binary(
            syntax::BinaryOp::Mult,
            one,
            Box::new(syntax::Expr::Binary(syntax::BinaryOp::Add, two, three)),
        )),
        Box::new(syntax::Expr::Binary(
            syntax::BinaryOp::Div,
            four,
            Box::new(syntax::Expr::Binary(syntax::BinaryOp::Add, five, six)),
        )),
    );

    assert_eq!(
        expr(Span::new("1 * (2 + 3) + 4 / (5 + 6);")),
        Ok((Span::new(";"), expected.clone()))
    );
}

#[test]
fn parse_complex_expr() {
    let input = Span::new("normalize((inverse(view) * vec4(ray.dir, 0.)).xyz);");
    let zero = syntax::Expr::FloatConst(0.);
    let ray = syntax::Expr::Variable(("ray").into());
    let raydir = syntax::Expr::Dot(Box::new(ray), ("dir").into());
    let vec4 = syntax::Expr::FunCall(
        syntax::FunIdentifier::Identifier(("vec4").into()),
        vec![raydir, zero],
    );
    let view = syntax::Expr::Variable(("view").into());
    let iview = syntax::Expr::FunCall(
        syntax::FunIdentifier::Identifier(("inverse").into()),
        vec![view],
    );
    let mul = syntax::Expr::Binary(syntax::BinaryOp::Mult, Box::new(iview), Box::new(vec4));
    let xyz = syntax::Expr::Dot(Box::new(mul), ("xyz").into());
    let normalize = syntax::Expr::FunCall(
        syntax::FunIdentifier::Identifier(("normalize").into()),
        vec![xyz],
    );
    let expected = normalize;

    assert_eq!(expr(input), Ok((Span::new(";"), expected)));
}

#[test]
fn parse_function_identifier_typename() {
    let expected = syntax::FunIdentifier::Identifier(("foo").into());
    assert_eq!(
        function_identifier(Span::new("foo(")),
        Ok((Span::new("("), expected.clone()))
    );
    assert_eq!(
        function_identifier(Span::new("foo\n\t(")),
        Ok((Span::new("("), expected.clone()))
    );
    assert_eq!(
        function_identifier(Span::new("foo\n (")),
        Ok((Span::new("("), expected))
    );
}

#[test]
fn parse_function_identifier_cast() {
    let expected = syntax::FunIdentifier::Identifier(("vec3").into());
    assert_eq!(
        function_identifier(Span::new("vec3(")),
        Ok((Span::new("("), expected.clone()))
    );
    assert_eq!(
        function_identifier(Span::new("vec3 (")),
        Ok((Span::new("("), expected.clone()))
    );
    assert_eq!(
        function_identifier(Span::new("vec3\t\n\n \t (")),
        Ok((Span::new("("), expected))
    );
}

#[test]
fn parse_function_identifier_cast_array_unsized() {
    let expected = syntax::FunIdentifier::Expr(Box::new(syntax::Expr::Bracket(
        Box::new(syntax::Expr::Variable(("vec3").into())),
        syntax::ArraySpecifier {
            dimensions: syntax::NonEmpty(vec![syntax::ArraySpecifierDimension::Unsized]),
        },
    )));

    assert_eq!(
        function_identifier(Span::new("vec3[](")),
        Ok((Span::new("("), expected.clone()))
    );
    assert_eq!(
        function_identifier(Span::new("vec3  [\t\n](")),
        Ok((Span::new("("), expected))
    );
}

#[test]
fn parse_function_identifier_cast_array_sized() {
    let expected = syntax::FunIdentifier::Expr(Box::new(syntax::Expr::Bracket(
        Box::new(syntax::Expr::Variable(("vec3").into())),
        syntax::ArraySpecifier {
            dimensions: syntax::NonEmpty(vec![syntax::ArraySpecifierDimension::ExplicitlySized(
                Box::new(syntax::Expr::IntConst(12)),
            )]),
        },
    )));

    assert_eq!(
        function_identifier(Span::new("vec3[12](")),
        Ok((Span::new("("), expected.clone()))
    );
    assert_eq!(
        function_identifier(Span::new("vec3  [\t 12\n](")),
        Ok((Span::new("("), expected))
    );
}

#[test]
fn parse_void() {
    assert_eq!(void(Span::new("void ")), Ok((Span::new(" "), ())));
}

#[test]
fn parse_assignment_op_equal() {
    assert_eq!(
        assignment_op(Span::new("= ")),
        Ok((Span::new(" "), syntax::AssignmentOp::Equal))
    );
}

#[test]
fn parse_assignment_op_mult() {
    assert_eq!(
        assignment_op(Span::new("*= ")),
        Ok((Span::new(" "), syntax::AssignmentOp::Mult))
    );
}

#[test]
fn parse_assignment_op_div() {
    assert_eq!(
        assignment_op(Span::new("/= ")),
        Ok((Span::new(" "), syntax::AssignmentOp::Div))
    );
}

#[test]
fn parse_assignment_op_mod() {
    assert_eq!(
        assignment_op(Span::new("%= ")),
        Ok((Span::new(" "), syntax::AssignmentOp::Mod))
    );
}

#[test]
fn parse_assignment_op_add() {
    assert_eq!(
        assignment_op(Span::new("+= ")),
        Ok((Span::new(" "), syntax::AssignmentOp::Add))
    );
}

#[test]
fn parse_assignment_op_sub() {
    assert_eq!(
        assignment_op(Span::new("-= ")),
        Ok((Span::new(" "), syntax::AssignmentOp::Sub))
    );
}

#[test]
fn parse_assignment_op_lshift() {
    assert_eq!(
        assignment_op(Span::new("<<= ")),
        Ok((Span::new(" "), syntax::AssignmentOp::LShift))
    );
}

#[test]
fn parse_assignment_op_rshift() {
    assert_eq!(
        assignment_op(Span::new(">>= ")),
        Ok((Span::new(" "), syntax::AssignmentOp::RShift))
    );
}

#[test]
fn parse_assignment_op_and() {
    assert_eq!(
        assignment_op(Span::new("&= ")),
        Ok((Span::new(" "), syntax::AssignmentOp::And))
    );
}

#[test]
fn parse_assignment_op_xor() {
    assert_eq!(
        assignment_op(Span::new("^= ")),
        Ok((Span::new(" "), syntax::AssignmentOp::Xor))
    );
}

#[test]
fn parse_assignment_op_or() {
    assert_eq!(
        assignment_op(Span::new("|= ")),
        Ok((Span::new(" "), syntax::AssignmentOp::Or))
    );
}

#[test]
fn parse_expr_statement() {
    let expected = Some(syntax::Expr::Assignment(
        Box::new(syntax::Expr::Variable(("foo").into())),
        syntax::AssignmentOp::Equal,
        Box::new(syntax::Expr::FloatConst(314.)),
    ));

    assert_eq!(
        expr_statement(Span::new("foo = 314.f;")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        expr_statement(Span::new("foo=314.f;")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        expr_statement(Span::new("foo\n\t=  \n314.f;")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_declaration_function_prototype() {
    let rt = syntax::FullySpecifiedType {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::Vec3,
            array_specifier: None,
        },
    };
    let arg0_ty = syntax::TypeSpecifier {
        ty: syntax::TypeSpecifierNonArray::Vec2,
        array_specifier: None,
    };
    let arg0 = syntax::FunctionParameterDeclaration::Unnamed(None, arg0_ty);
    let qual_spec = syntax::TypeQualifierSpec::Storage(syntax::StorageQualifier::Out);
    let qual = syntax::TypeQualifier {
        qualifiers: syntax::NonEmpty(vec![qual_spec]),
    };
    let arg1 = syntax::FunctionParameterDeclaration::Named(
        Some(qual),
        syntax::FunctionParameterDeclarator {
            ty: syntax::TypeSpecifier {
                ty: syntax::TypeSpecifierNonArray::Float,
                array_specifier: None,
            },
            ident: ("the_arg").into(),
        },
    );
    let fp = syntax::FunctionPrototype {
        ty: rt,
        name: ("foo").into(),
        parameters: vec![arg0, arg1],
    };
    let expected = syntax::Declaration::FunctionPrototype(fp);

    assert_eq!(
        declaration(Span::new("vec3 foo(vec2, out float the_arg);")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        declaration(Span::new("vec3 \nfoo ( vec2\n, out float \n\tthe_arg )\n;")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        declaration(Span::new("vec3 foo(vec2,out float the_arg);")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_declaration_init_declarator_list_single() {
    let ty = syntax::FullySpecifiedType {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::Int,
            array_specifier: None,
        },
    };
    let sd = syntax::SingleDeclaration {
        ty,
        name: Some(("foo").into()),
        array_specifier: None,
        initializer: Some(syntax::Initializer::Simple(Box::new(
            syntax::Expr::IntConst(34),
        ))),
    };
    let idl = syntax::InitDeclaratorList {
        head: sd,
        tail: Vec::new(),
    };
    let expected = syntax::Declaration::InitDeclaratorList(idl);

    assert_eq!(
        declaration(Span::new("int foo = 34;")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        declaration(Span::new("int foo=34;")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        declaration(Span::new("int    \t  \nfoo =\t34  ;")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_declaration_init_declarator_list_complex() {
    let ty = syntax::FullySpecifiedType {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::Int,
            array_specifier: None,
        },
    };
    let sd = syntax::SingleDeclaration {
        ty,
        name: Some(("foo").into()),
        array_specifier: None,
        initializer: Some(syntax::Initializer::Simple(Box::new(
            syntax::Expr::IntConst(34),
        ))),
    };
    let sdnt = syntax::SingleDeclarationNoType {
        ident: ("bar").into(),
        initializer: Some(syntax::Initializer::Simple(Box::new(
            syntax::Expr::IntConst(12),
        ))),
    };
    let expected = syntax::Declaration::InitDeclaratorList(syntax::InitDeclaratorList {
        head: sd,
        tail: vec![sdnt],
    });

    assert_eq!(
        declaration(Span::new("int foo = 34, bar = 12;")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        declaration(Span::new("int foo=34,bar=12;")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        declaration(Span::new("int    \t  \nfoo =\t34 \n,\tbar=      12\n ;")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_declaration_precision_low() {
    let qual = syntax::PrecisionQualifier::Low;
    let ty = syntax::TypeSpecifier {
        ty: syntax::TypeSpecifierNonArray::Float,
        array_specifier: None,
    };
    let expected = syntax::Declaration::Precision(qual, ty);

    assert_eq!(
        declaration(Span::new("precision lowp float;")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_declaration_precision_medium() {
    let qual = syntax::PrecisionQualifier::Medium;
    let ty = syntax::TypeSpecifier {
        ty: syntax::TypeSpecifierNonArray::Float,
        array_specifier: None,
    };
    let expected = syntax::Declaration::Precision(qual, ty);

    assert_eq!(
        declaration(Span::new("precision mediump float;")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_declaration_precision_high() {
    let qual = syntax::PrecisionQualifier::High;
    let ty = syntax::TypeSpecifier {
        ty: syntax::TypeSpecifierNonArray::Float,
        array_specifier: None,
    };
    let expected = syntax::Declaration::Precision(qual, ty);

    assert_eq!(
        declaration(Span::new("precision highp float;")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_declaration_uniform_block() {
    let qual_spec = syntax::TypeQualifierSpec::Storage(syntax::StorageQualifier::Uniform);
    let qual = syntax::TypeQualifier {
        qualifiers: syntax::NonEmpty(vec![qual_spec]),
    };
    let f0 = syntax::StructFieldSpecifier {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::Float,
            array_specifier: None,
        },
        identifiers: syntax::NonEmpty(vec![("a").into()]),
    };
    let f1 = syntax::StructFieldSpecifier {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::Vec3,
            array_specifier: None,
        },
        identifiers: syntax::NonEmpty(vec![("b").into()]),
    };
    let f2 = syntax::StructFieldSpecifier {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::TypeName(("foo").into()),
            array_specifier: None,
        },
        identifiers: syntax::NonEmpty(vec![("c").into(), ("d").into()]),
    };
    let expected = syntax::Declaration::Block(syntax::Block {
        qualifier: qual,
        name: ("UniformBlockTest").into(),
        fields: vec![f0, f1, f2],
        identifier: None,
    });

    assert_eq!(
        declaration(Span::new(
            "uniform UniformBlockTest { float a; vec3 b; foo c, d; };"
        )),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(declaration(Span::new("uniform   \nUniformBlockTest\n {\n \t float   a  \n; \nvec3 b\n; foo \nc\n, \nd\n;\n }\n\t\n\t\t \t;")), Ok((Span::new(""), expected)));
}

#[test]
fn parse_declaration_buffer_block() {
    let qual_spec = syntax::TypeQualifierSpec::Storage(syntax::StorageQualifier::Buffer);
    let qual = syntax::TypeQualifier {
        qualifiers: syntax::NonEmpty(vec![qual_spec]),
    };
    let f0 = syntax::StructFieldSpecifier {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::Float,
            array_specifier: None,
        },
        identifiers: syntax::NonEmpty(vec![("a").into()]),
    };
    let f1 = syntax::StructFieldSpecifier {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::Vec3,
            array_specifier: None,
        },
        identifiers: syntax::NonEmpty(vec![syntax::ArrayedIdentifier::new(
            ("b"),
            Some(syntax::ArraySpecifier {
                dimensions: syntax::NonEmpty(vec![syntax::ArraySpecifierDimension::Unsized]),
            }),
        )]),
    };
    let f2 = syntax::StructFieldSpecifier {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::TypeName(("foo").into()),
            array_specifier: None,
        },
        identifiers: syntax::NonEmpty(vec![("c").into(), ("d").into()]),
    };
    let expected = syntax::Declaration::Block(syntax::Block {
        qualifier: qual,
        name: ("UniformBlockTest").into(),
        fields: vec![f0, f1, f2],
        identifier: None,
    });

    assert_eq!(
        declaration(Span::new(
            "buffer UniformBlockTest { float a; vec3 b[]; foo c, d; };"
        )),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(declaration(Span::new("buffer   \nUniformBlockTest\n {\n \t float   a  \n; \nvec3 b   [   ]\n; foo \nc\n, \nd\n;\n }\n\t\n\t\t \t;")), Ok((Span::new(""), expected)));
}

#[test]
fn parse_selection_statement_if() {
    let cond = syntax::Expr::Binary(
        syntax::BinaryOp::LT,
        Box::new(syntax::Expr::Variable(("foo").into())),
        Box::new(syntax::Expr::IntConst(10)),
    );
    let ret = Box::new(syntax::Expr::BoolConst(false));
    let st = syntax::Statement::Simple(Box::new(syntax::SimpleStatement::Jump(
        syntax::JumpStatement::Return(Some(ret)),
    )));
    let body = syntax::Statement::Compound(Box::new(syntax::CompoundStatement {
        statement_list: vec![st],
    }));
    let rest = syntax::SelectionRestStatement::Statement(Box::new(body));
    let expected = syntax::SelectionStatement {
        cond: Box::new(cond),
        rest,
    };

    assert_eq!(
        selection_statement(Span::new("if (foo < 10) { return false; }K")),
        Ok((Span::new("K"), expected.clone()))
    );
    assert_eq!(
        selection_statement(Span::new("if \n(foo<10\n) \t{return false;}K")),
        Ok((Span::new("K"), expected))
    );
}

#[test]
fn parse_selection_statement_if_else() {
    let cond = syntax::Expr::Binary(
        syntax::BinaryOp::LT,
        Box::new(syntax::Expr::Variable(("foo").into())),
        Box::new(syntax::Expr::IntConst(10)),
    );
    let if_ret = Box::new(syntax::Expr::FloatConst(0.));
    let if_st = syntax::Statement::Simple(Box::new(syntax::SimpleStatement::Jump(
        syntax::JumpStatement::Return(Some(if_ret)),
    )));
    let if_body = syntax::Statement::Compound(Box::new(syntax::CompoundStatement {
        statement_list: vec![if_st],
    }));
    let else_ret = Box::new(syntax::Expr::Variable(("foo").into()));
    let else_st = syntax::Statement::Simple(Box::new(syntax::SimpleStatement::Jump(
        syntax::JumpStatement::Return(Some(else_ret)),
    )));
    let else_body = syntax::Statement::Compound(Box::new(syntax::CompoundStatement {
        statement_list: vec![else_st],
    }));
    let rest = syntax::SelectionRestStatement::Else(Box::new(if_body), Box::new(else_body));
    let expected = syntax::SelectionStatement {
        cond: Box::new(cond),
        rest,
    };

    assert_eq!(
        selection_statement(Span::new(
            "if (foo < 10) { return 0.f; } else { return foo; }"
        )),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        selection_statement(Span::new(
            "if \n(foo<10\n) \t{return 0.f\t;\n\n}\n else{\n\t return foo   ;}"
        )),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_switch_statement_empty() {
    let head = Box::new(syntax::Expr::Variable(("foo").into()));
    let expected = syntax::SwitchStatement {
        head,
        body: Vec::new(),
    };

    assert_eq!(
        switch_statement(Span::new("switch (foo) {}")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        switch_statement(Span::new("switch(foo){}")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        switch_statement(Span::new("switch\n\n (  foo  \t   \n) { \n\n   }")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_switch_statement_cases() {
    let head = Box::new(syntax::Expr::Variable(("foo").into()));
    let case0 = syntax::Statement::Simple(Box::new(syntax::SimpleStatement::CaseLabel(
        syntax::CaseLabel::Case(Box::new(syntax::Expr::IntConst(0))),
    )));
    let case1 = syntax::Statement::Simple(Box::new(syntax::SimpleStatement::CaseLabel(
        syntax::CaseLabel::Case(Box::new(syntax::Expr::IntConst(1))),
    )));
    let ret = syntax::Statement::Simple(Box::new(syntax::SimpleStatement::Jump(
        syntax::JumpStatement::Return(Some(Box::new(syntax::Expr::UIntConst(12)))),
    )));
    let expected = syntax::SwitchStatement {
        head,
        body: vec![case0, case1, ret],
    };

    assert_eq!(
        switch_statement(Span::new("switch (foo) { case 0: case 1: return 12u; }")),
        Ok((Span::new(""), expected.clone()))
    );
}

#[test]
fn parse_case_label_def() {
    assert_eq!(
        case_label(Span::new("default:")),
        Ok((Span::new(""), syntax::CaseLabel::Def))
    );
    assert_eq!(
        case_label(Span::new("default   :")),
        Ok((Span::new(""), syntax::CaseLabel::Def))
    );
}

#[test]
fn parse_case_label() {
    let expected = syntax::CaseLabel::Case(Box::new(syntax::Expr::IntConst(3)));

    assert_eq!(
        case_label(Span::new("case 3:")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        case_label(Span::new("case\n\t 3   :")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_iteration_statement_while_empty() {
    let cond = syntax::Condition::Expr(Box::new(syntax::Expr::Binary(
        syntax::BinaryOp::GTE,
        Box::new(syntax::Expr::Variable(("a").into())),
        Box::new(syntax::Expr::Variable(("b").into())),
    )));
    let st = syntax::Statement::Compound(Box::new(syntax::CompoundStatement {
        statement_list: Vec::new(),
    }));
    let expected = syntax::IterationStatement::While(cond, Box::new(st));

    assert_eq!(
        iteration_statement(Span::new("while (a >= b) {}")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        iteration_statement(Span::new("while(a>=b){}")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        iteration_statement(Span::new("while (  a >=\n\tb  )\t  {   \n}")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_iteration_statement_do_while_empty() {
    let st = syntax::Statement::Compound(Box::new(syntax::CompoundStatement {
        statement_list: Vec::new(),
    }));
    let cond = Box::new(syntax::Expr::Binary(
        syntax::BinaryOp::GTE,
        Box::new(syntax::Expr::Variable(("a").into())),
        Box::new(syntax::Expr::Variable(("b").into())),
    ));
    let expected = syntax::IterationStatement::DoWhile(Box::new(st), cond);

    assert_eq!(
        iteration_statement(Span::new("do {} while (a >= b);")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        iteration_statement(Span::new("do{}while(a>=b);")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        iteration_statement(Span::new("do \n {\n} while (  a >=\n\tb  )\t  \n;")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_iteration_statement_for_empty() {
    let init = syntax::ForInitStatement::Declaration(Box::new(
        syntax::Declaration::InitDeclaratorList(syntax::InitDeclaratorList {
            head: syntax::SingleDeclaration {
                ty: syntax::FullySpecifiedType {
                    qualifier: None,
                    ty: syntax::TypeSpecifier {
                        ty: syntax::TypeSpecifierNonArray::Float,
                        array_specifier: None,
                    },
                },
                name: Some(("i").into()),
                array_specifier: None,
                initializer: Some(syntax::Initializer::Simple(Box::new(
                    syntax::Expr::FloatConst(0.),
                ))),
            },
            tail: Vec::new(),
        }),
    ));
    let rest = syntax::ForRestStatement {
        condition: Some(syntax::Condition::Expr(Box::new(syntax::Expr::Binary(
            syntax::BinaryOp::LTE,
            Box::new(syntax::Expr::Variable(("i").into())),
            Box::new(syntax::Expr::FloatConst(10.)),
        )))),
        post_expr: Some(Box::new(syntax::Expr::Unary(
            syntax::UnaryOp::Inc,
            Box::new(syntax::Expr::Variable(("i").into())),
        ))),
    };
    let st = syntax::Statement::Compound(Box::new(syntax::CompoundStatement {
        statement_list: Vec::new(),
    }));
    let expected = syntax::IterationStatement::For(init, rest, Box::new(st));

    assert_eq!(
        iteration_statement(Span::new("for (float i = 0.f; i <= 10.f; ++i) {}")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        iteration_statement(Span::new("for(float i=0.f;i<=10.f;++i){}")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        iteration_statement(Span::new(
            "for\n\t (  \t\n\nfloat \ni \t=\n0.f\n;\ni\t<=  10.f; \n++i\n)\n{\n}"
        )),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_jump_continue() {
    assert_eq!(
        jump_statement(Span::new("continue;")),
        Ok((Span::new(""), syntax::JumpStatement::Continue))
    );
}

#[test]
fn parse_jump_break() {
    assert_eq!(
        jump_statement(Span::new("break;")),
        Ok((Span::new(""), syntax::JumpStatement::Break))
    );
}

#[test]
fn parse_jump_return() {
    let expected = syntax::JumpStatement::Return(Some(Box::new(syntax::Expr::IntConst(3))));
    assert_eq!(
        jump_statement(Span::new("return 3;")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_jump_empty_return() {
    let expected = syntax::SimpleStatement::Jump(syntax::JumpStatement::Return(None));
    assert_eq!(
        simple_statement(Span::new("return;")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_jump_discard() {
    assert_eq!(
        jump_statement(Span::new("discard;")),
        Ok((Span::new(""), syntax::JumpStatement::Discard))
    );
}

#[test]
fn parse_simple_statement_return() {
    let e = syntax::Expr::BoolConst(false);
    let expected = syntax::SimpleStatement::Jump(syntax::JumpStatement::Return(Some(Box::new(e))));

    assert_eq!(
        simple_statement(Span::new("return false;")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_compound_statement_empty() {
    let expected = syntax::CompoundStatement {
        statement_list: Vec::new(),
    };

    assert_eq!(
        compound_statement(Span::new("{}")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_compound_statement() {
    let st0 = syntax::Statement::Simple(Box::new(syntax::SimpleStatement::Selection(
        syntax::SelectionStatement {
            cond: Box::new(syntax::Expr::BoolConst(true)),
            rest: syntax::SelectionRestStatement::Statement(Box::new(syntax::Statement::Compound(
                Box::new(syntax::CompoundStatement {
                    statement_list: Vec::new(),
                }),
            ))),
        },
    )));
    let st1 = syntax::Statement::Simple(Box::new(syntax::SimpleStatement::Declaration(
        syntax::Declaration::InitDeclaratorList(syntax::InitDeclaratorList {
            head: syntax::SingleDeclaration {
                ty: syntax::FullySpecifiedType {
                    qualifier: None,
                    ty: syntax::TypeSpecifier {
                        ty: syntax::TypeSpecifierNonArray::ISampler3D,
                        array_specifier: None,
                    },
                },
                name: Some(("x").into()),
                array_specifier: None,
                initializer: None,
            },
            tail: Vec::new(),
        }),
    )));
    let st2 = syntax::Statement::Simple(Box::new(syntax::SimpleStatement::Jump(
        syntax::JumpStatement::Return(Some(Box::new(syntax::Expr::IntConst(42)))),
    )));
    let expected = syntax::CompoundStatement {
        statement_list: vec![st0, st1, st2],
    };

    assert_eq!(
        compound_statement(Span::new("{ if (true) {} isampler3D x; return 42 ; }")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        compound_statement(Span::new("{if(true){}isampler3D x;return 42;}")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_function_definition() {
    let rt = syntax::FullySpecifiedType {
        qualifier: None,
        ty: syntax::TypeSpecifier {
            ty: syntax::TypeSpecifierNonArray::IImage2DArray,
            array_specifier: None,
        },
    };
    let fp = syntax::FunctionPrototype {
        ty: rt,
        name: ("foo").into(),
        parameters: Vec::new(),
    };
    let st0 = syntax::Statement::Simple(Box::new(syntax::SimpleStatement::Jump(
        syntax::JumpStatement::Return(Some(Box::new(syntax::Expr::Variable(("bar").into())))),
    )));
    let expected = syntax::FunctionDefinition {
        prototype: fp,
        statement: syntax::CompoundStatement {
            statement_list: vec![st0],
        },
    };

    assert_eq!(
        function_definition(Span::new("iimage2DArray foo() { return bar; }")),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        function_definition(Span::new(
            "iimage2DArray \tfoo\n()\n \n{\n return \nbar\n;}"
        )),
        Ok((Span::new(""), expected.clone()))
    );
    assert_eq!(
        function_definition(Span::new("iimage2DArray foo(){return bar;}")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_buffer_block_0() {
    let src = include_str!("../data/tests/buffer_block_0.glsl");
    let main_fn = syntax::ExternalDeclaration::FunctionDefinition(syntax::FunctionDefinition {
        prototype: syntax::FunctionPrototype {
            ty: syntax::FullySpecifiedType {
                qualifier: None,
                ty: syntax::TypeSpecifier {
                    ty: syntax::TypeSpecifierNonArray::Void,
                    array_specifier: None,
                },
            },
            name: ("main").into(),
            parameters: Vec::new(),
        },
        statement: syntax::CompoundStatement {
            statement_list: Vec::new(),
        },
    });
    let buffer_block =
        syntax::ExternalDeclaration::Declaration(syntax::Declaration::Block(syntax::Block {
            qualifier: syntax::TypeQualifier {
                qualifiers: syntax::NonEmpty(vec![syntax::TypeQualifierSpec::Storage(
                    syntax::StorageQualifier::Buffer,
                )]),
            },
            name: ("Foo").into(),
            fields: vec![syntax::StructFieldSpecifier {
                qualifier: None,
                ty: syntax::TypeSpecifier {
                    ty: syntax::TypeSpecifierNonArray::TypeName(("char").into()),
                    array_specifier: None,
                },
                identifiers: syntax::NonEmpty(vec![syntax::ArrayedIdentifier::new(
                    ("tiles"),
                    Some(syntax::ArraySpecifier {
                        dimensions: syntax::NonEmpty(vec![
                            syntax::ArraySpecifierDimension::Unsized,
                        ]),
                    }),
                )]),
            }],
            identifier: Some(("main_tiles").into()),
        }));
    let expected = syntax::TranslationUnit(syntax::NonEmpty(vec![buffer_block, main_fn]));

    assert_eq!(
        translation_unit(Span::new(src)),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_layout_buffer_block_0() {
    let src = include_str!("../data/tests/layout_buffer_block_0.glsl");
    let layout = syntax::LayoutQualifier {
        ids: syntax::NonEmpty(vec![
            syntax::LayoutQualifierSpec::Identifier(
                ("set").into(),
                Some(Box::new(syntax::Expr::IntConst(0))),
            ),
            syntax::LayoutQualifierSpec::Identifier(
                ("binding").into(),
                Some(Box::new(syntax::Expr::IntConst(0))),
            ),
        ]),
    };
    let type_qual = syntax::TypeQualifier {
        qualifiers: syntax::NonEmpty(vec![
            syntax::TypeQualifierSpec::Layout(layout),
            syntax::TypeQualifierSpec::Storage(syntax::StorageQualifier::Buffer),
        ]),
    };
    let block =
        syntax::ExternalDeclaration::Declaration(syntax::Declaration::Block(syntax::Block {
            qualifier: type_qual,
            name: ("Foo").into(),
            fields: vec![syntax::StructFieldSpecifier {
                qualifier: None,
                ty: syntax::TypeSpecifier {
                    ty: syntax::TypeSpecifierNonArray::TypeName(("char").into()),
                    array_specifier: None,
                },
                identifiers: syntax::NonEmpty(vec![("a").into()]),
            }],
            identifier: Some(("foo").into()),
        }));

    let expected = syntax::TranslationUnit(syntax::NonEmpty(vec![block]));

    assert_eq!(
        translation_unit(Span::new(src)),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_pp_space0() {
    assert_eq!(
        pp_space0(Span::new("   \\\n  ")),
        Ok((Span::new(""), Span::new("   \\\n  ")))
    );
    assert_eq!(pp_space0(Span::new("")), Ok((Span::new(""), Span::new(""))));
}

#[test]
fn parse_pp_version_number() {
    assert_eq!(
        pp_version_number(Span::new("450")),
        Ok((Span::new(""), 450))
    );
}

#[test]
fn parse_pp_version_profile() {
    assert_eq!(
        pp_version_profile(Span::new("core")),
        Ok((Span::new(""), syntax::PreprocessorVersionProfile::Core))
    );
    assert_eq!(
        pp_version_profile(Span::new("compatibility")),
        Ok((
            Span::new(""),
            syntax::PreprocessorVersionProfile::Compatibility
        ))
    );
    assert_eq!(
        pp_version_profile(Span::new("es")),
        Ok((Span::new(""), syntax::PreprocessorVersionProfile::ES))
    );
}

#[test]
fn parse_pp_version() {
    assert_eq!(
        preprocessor(Span::new("#version 450\n")),
        Ok((
            Span::new(""),
            syntax::Preprocessor::Version(syntax::PreprocessorVersion {
                version: 450,
                profile: None,
            })
        ))
    );

    assert_eq!(
        preprocessor(Span::new("#version 450 core\n")),
        Ok((
            Span::new(""),
            syntax::Preprocessor::Version(syntax::PreprocessorVersion {
                version: 450,
                profile: Some(syntax::PreprocessorVersionProfile::Core)
            })
        ))
    );
}

#[test]
fn parse_pp_version_newline() {
    assert_eq!(
        preprocessor(Span::new("#version 450\n")),
        Ok((
            Span::new(""),
            syntax::Preprocessor::Version(syntax::PreprocessorVersion {
                version: 450,
                profile: None,
            })
        ))
    );

    assert_eq!(
        preprocessor(Span::new("#version 450 core\n")),
        Ok((
            Span::new(""),
            syntax::Preprocessor::Version(syntax::PreprocessorVersion {
                version: 450,
                profile: Some(syntax::PreprocessorVersionProfile::Core)
            })
        ))
    );
}

#[test]
fn parse_pp_define() {
    let expect = |v: &str| {
        Ok((
            Span::new(""),
            syntax::Preprocessor::Define(syntax::PreprocessorDefine::ObjectLike {
                ident: ("test").into(),
                value: v.to_owned(),
            }),
        ))
    };

    assert_eq!(preprocessor(Span::new("#define test 1.0")), expect(("1.0")));
    assert_eq!(
        preprocessor(Span::new("#define test \\\n   1.0")),
        expect(("1.0"))
    );
    assert_eq!(
        preprocessor(Span::new("#define test 1.0\n")),
        expect(("1.0"))
    );

    assert_eq!(
        preprocessor(Span::new("#define test123 .0f\n")),
        Ok((
            Span::new(""),
            syntax::Preprocessor::Define(syntax::PreprocessorDefine::ObjectLike {
                ident: ("test123").into(),
                value: (".0f").to_owned()
            })
        ))
    );

    assert_eq!(
        preprocessor(Span::new("#define test 1\n")),
        Ok((
            Span::new(""),
            syntax::Preprocessor::Define(syntax::PreprocessorDefine::ObjectLike {
                ident: ("test").into(),
                value: ("1").to_owned()
            })
        ))
    );
}

#[test]
fn parse_pp_define_with_args() {
    let expected = syntax::Preprocessor::Define(syntax::PreprocessorDefine::FunctionLike {
        ident: ("add").into(),
        args: vec![
            syntax::Identifier::new(("x")).unwrap(),
            syntax::Identifier::new(("y")).unwrap(),
        ],
        value: ("(x + y)").to_owned(),
    });

    assert_eq!(
        preprocessor(Span::new("#define \\\n add(x, y) \\\n (x + y)")),
        Ok((Span::new(""), expected.clone()))
    );

    assert_eq!(
        preprocessor(Span::new("#define \\\n add(  x, y  ) \\\n (x + y)")),
        Ok((Span::new(""), expected))
    );
}

#[test]
fn parse_pp_else() {
    assert_eq!(
        preprocessor(Span::new("#    else\n")),
        Ok((Span::new(""), syntax::Preprocessor::Else))
    );
}

#[test]
fn parse_pp_elseif() {
    assert_eq!(
        preprocessor(Span::new("#   elseif \\\n42\n")),
        Ok((
            Span::new(""),
            syntax::Preprocessor::ElseIf(syntax::PreprocessorElseIf {
                condition: ("42").to_owned()
            })
        ))
    );
}

#[test]
fn parse_pp_endif() {
    assert_eq!(
        preprocessor(Span::new("#\\\nendif")),
        Ok((Span::new(""), syntax::Preprocessor::EndIf))
    );
}

#[test]
fn parse_pp_error() {
    assert_eq!(
        preprocessor(Span::new("#error \\\n     some message")),
        Ok((
            Span::new(""),
            syntax::Preprocessor::Error(syntax::PreprocessorError {
                message: ("some message").to_owned()
            })
        ))
    );
}

#[test]
fn parse_pp_if() {
    assert_eq!(
        preprocessor(Span::new("# \\\nif 42")),
        Ok((
            Span::new(""),
            syntax::Preprocessor::If(syntax::PreprocessorIf {
                condition: ("42").to_owned()
            })
        ))
    );
}

#[test]
fn parse_pp_ifdef() {
    assert_eq!(
        preprocessor(Span::new("#ifdef       FOO\n")),
        Ok((
            Span::new(""),
            syntax::Preprocessor::IfDef(syntax::PreprocessorIfDef {
                ident: syntax::Identifier(("FOO").to_owned())
            })
        ))
    );
}

#[test]
fn parse_pp_ifndef() {
    assert_eq!(
        preprocessor(Span::new("#\\\nifndef \\\n   FOO\n")),
        Ok((
            Span::new(""),
            syntax::Preprocessor::IfNDef(syntax::PreprocessorIfNDef {
                ident: syntax::Identifier(("FOO").to_owned())
            })
        ))
    );
}

#[test]
fn parse_pp_include() {
    assert_eq!(
        preprocessor(Span::new("#include <filename>\n")),
        Ok((
            Span::new(""),
            syntax::Preprocessor::Include(syntax::PreprocessorInclude {
                path: syntax::Path::Absolute(("filename").to_owned())
            })
        ))
    );

    #[test]
    fn parse_pp_line() {
        assert_eq!(
            preprocessor(Span::new("#   line \\\n2\n")),
            Ok((
                Span::new(""),
                syntax::Preprocessor::Line(syntax::PreprocessorLine {
                    line: 2,
                    source_string_number: None,
                })
            ))
        );

        assert_eq!(
            preprocessor(Span::new("#line 2 \\\n 4\n")),
            Ok((
                Span::new(""),
                syntax::Preprocessor::Line(syntax::PreprocessorLine {
                    line: 2,
                    source_string_number: Some(4),
                })
            ))
        );
    }

    #[test]
    fn parse_pp_pragma() {
        assert_eq!(
            preprocessor(Span::new("#\\\npragma  some   flag")),
            Ok((
                Span::new(""),
                syntax::Preprocessor::Pragma(syntax::PreprocessorPragma {
                    command: ("some   flag").to_owned()
                })
            ))
        );
    }

    #[test]
    fn parse_pp_undef() {
        assert_eq!(
            preprocessor(Span::new("# undef \\\n FOO")),
            Ok((
                Span::new(""),
                syntax::Preprocessor::Undef(syntax::PreprocessorUndef {
                    name: syntax::Identifier(("FOO").to_owned())
                })
            ))
        );
    }

    #[test]
    fn parse_pp_extension_name() {
        assert_eq!(
            pp_extension_name(Span::new("all")),
            Ok((Span::new(""), syntax::PreprocessorExtensionName::All))
        );
        assert_eq!(
            pp_extension_name(Span::new("GL_foobar_extension ")),
            Ok((
                Span::new(" "),
                syntax::PreprocessorExtensionName::Specific(("GL_foobar_extension").to_owned())
            ))
        );
    }

    #[test]
    fn parse_pp_extension_behavior() {
        assert_eq!(
            pp_extension_behavior(Span::new("require")),
            Ok((
                Span::new(""),
                syntax::PreprocessorExtensionBehavior::Require
            ))
        );
        assert_eq!(
            pp_extension_behavior(Span::new("enable")),
            Ok((Span::new(""), syntax::PreprocessorExtensionBehavior::Enable))
        );
        assert_eq!(
            pp_extension_behavior(Span::new("warn")),
            Ok((Span::new(""), syntax::PreprocessorExtensionBehavior::Warn))
        );
        assert_eq!(
            pp_extension_behavior(Span::new("disable")),
            Ok((
                Span::new(""),
                syntax::PreprocessorExtensionBehavior::Disable
            ))
        );
    }

    #[test]
    fn parse_pp_extension() {
        assert_eq!(
            preprocessor(Span::new("#extension all: require\n")),
            Ok((
                Span::new(""),
                syntax::Preprocessor::Extension(syntax::PreprocessorExtension {
                    name: syntax::PreprocessorExtensionName::All,
                    behavior: Some(syntax::PreprocessorExtensionBehavior::Require)
                })
            ))
        );
    }

    #[test]
    fn parse_dot_field_expr_array() {
        let src = Span::new("a[0].xyz;");
        let expected = syntax::Expr::Dot(
            Box::new(syntax::Expr::Bracket(
                Box::new(syntax::Expr::Variable(("a").into())),
                syntax::ArraySpecifier {
                    dimensions: syntax::NonEmpty(vec![
                        syntax::ArraySpecifierDimension::ExplicitlySized(Box::new(
                            syntax::Expr::IntConst(0),
                        )),
                    ]),
                },
            )),
            ("xyz").into(),
        );

        assert_eq!(expr(src), Ok((Span::new(";"), expected)));
    }

    #[test]
    fn parse_dot_field_expr_statement() {
        let src =
            Span::new("vec3 v = smoothstep(vec3(border_width), vec3(0.0), v_barycenter).zyx;");
        let fun = syntax::FunIdentifier::Identifier(("smoothstep").into());
        let args = vec![
            syntax::Expr::FunCall(
                syntax::FunIdentifier::Identifier(("vec3").into()),
                vec![syntax::Expr::Variable(("border_width").into())],
            ),
            syntax::Expr::FunCall(
                syntax::FunIdentifier::Identifier(("vec3").into()),
                vec![syntax::Expr::FloatConst(0.)],
            ),
            syntax::Expr::Variable(("v_barycenter").into()),
        ];
        let ini = syntax::Initializer::Simple(Box::new(syntax::Expr::Dot(
            Box::new(syntax::Expr::FunCall(fun, args)),
            ("zyx").into(),
        )));
        let sd = syntax::SingleDeclaration {
            ty: syntax::FullySpecifiedType {
                qualifier: None,
                ty: syntax::TypeSpecifier {
                    ty: syntax::TypeSpecifierNonArray::Vec3,
                    array_specifier: None,
                },
            },
            name: Some(("v").into()),
            array_specifier: None,
            initializer: Some(ini),
        };
        let expected = syntax::Statement::Simple(Box::new(syntax::SimpleStatement::Declaration(
            syntax::Declaration::InitDeclaratorList(syntax::InitDeclaratorList {
                head: sd,
                tail: Vec::new(),
            }),
        )));

        assert_eq!(statement(src), Ok((Span::new(""), expected)));
    }

    #[test]
    fn parse_arrayed_identifier() {
        let expected = syntax::ArrayedIdentifier::new(
            ("foo"),
            syntax::ArraySpecifier {
                dimensions: syntax::NonEmpty(vec![syntax::ArraySpecifierDimension::Unsized]),
            },
        );

        assert_eq!(
            arrayed_identifier(Span::new("foo[]")),
            Ok((Span::new(""), expected.clone()))
        );
        assert_eq!(
            arrayed_identifier(Span::new("foo \t\n  [\n\t ]")),
            Ok((Span::new(""), expected))
        );
    }

    #[test]
    fn parse_nested_parens() {
        let start = std::time::Instant::now();
        parens_expr(Span::new("((((((((1.0f))))))))")).unwrap();
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 100, "{} ms", elapsed.as_millis());
    }
}
