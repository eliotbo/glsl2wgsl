//! GLSL parsing.
//!
//! Original AST from Dimitri Sabadie's glsl crate: https://github.com/phaazon/glsl

use crate::nom_helpers::{IResult2, ParseError, Span};
use crate::syntax;
use nom::Err as NomErr;

pub(crate) fn run_parser<P, T>(source: Span, parser: P) -> Result<T, ParseError>
where
    P: FnOnce(Span) -> IResult2<T>,
{
    match parser(source) {
        Ok((_, x)) => Ok(x),

        Err(e) => match e {
            NomErr::Incomplete(_) => Err(ParseError::new("incomplete parser".to_string(), source)),

            NomErr::Error(err) | NomErr::Failure(err) => Err(err),
        },
    }
}

pub trait Parse: Sized {
    fn parse<'a>(source: Span<'a>) -> Result<Self, ParseError<'a>>;
}

macro_rules! impl_parse {
    ($type_name:ty, $parser_name:ident) => {
        impl Parse for $type_name {
            fn parse<'a>(source: Span<'a>) -> Result<Self, ParseError<'a>> {
                run_parser(source, $crate::parsers_span::$parser_name)
            }
        }
    };
}

impl_parse!(syntax::Identifier, identifier);
impl_parse!(syntax::TypeSpecifierNonArray, type_specifier_non_array);
impl_parse!(syntax::TypeSpecifier, type_specifier);
impl_parse!(syntax::UnaryOp, unary_op);
impl_parse!(syntax::StructFieldSpecifier, struct_field_specifier);
impl_parse!(syntax::StructSpecifier, struct_specifier);
impl_parse!(syntax::StorageQualifier, storage_qualifier);
impl_parse!(syntax::LayoutQualifier, layout_qualifier);
impl_parse!(syntax::PrecisionQualifier, precision_qualifier);
impl_parse!(syntax::InterpolationQualifier, interpolation_qualifier);
impl_parse!(syntax::TypeQualifier, type_qualifier);
impl_parse!(syntax::TypeQualifierSpec, type_qualifier_spec);
impl_parse!(syntax::FullySpecifiedType, fully_specified_type);
impl_parse!(syntax::ArraySpecifier, array_specifier);
impl_parse!(syntax::Expr, expr);
impl_parse!(syntax::Declaration, declaration);
impl_parse!(syntax::FunctionPrototype, function_prototype);
impl_parse!(syntax::InitDeclaratorList, init_declarator_list);
impl_parse!(syntax::SingleDeclaration, single_declaration);
impl_parse!(syntax::Initializer, initializer);
impl_parse!(syntax::FunIdentifier, function_identifier);
impl_parse!(syntax::AssignmentOp, assignment_op);
impl_parse!(syntax::SimpleStatement, simple_statement);
impl_parse!(syntax::ExprStatement, expr_statement);
impl_parse!(syntax::SelectionStatement, selection_statement);
impl_parse!(syntax::SwitchStatement, switch_statement);
impl_parse!(syntax::CaseLabel, case_label);
impl_parse!(syntax::IterationStatement, iteration_statement);
impl_parse!(syntax::JumpStatement, jump_statement);
impl_parse!(syntax::Condition, condition);
impl_parse!(syntax::Statement, statement);
impl_parse!(syntax::CompoundStatement, compound_statement);
impl_parse!(syntax::FunctionDefinition, function_definition);
impl_parse!(syntax::ExternalDeclaration, external_declaration);
impl_parse!(syntax::TranslationUnit, translation_unit);
impl_parse!(syntax::Preprocessor, preprocessor);
impl_parse!(syntax::PreprocessorVersion, pp_version);
impl_parse!(syntax::PreprocessorVersionProfile, pp_version_profile);
impl_parse!(syntax::PreprocessorExtensionName, pp_extension_name);
impl_parse!(syntax::PreprocessorExtensionBehavior, pp_extension_behavior);
impl_parse!(syntax::PreprocessorExtension, pp_extension);
