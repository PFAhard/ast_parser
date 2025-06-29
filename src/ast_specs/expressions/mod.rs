mod assignment;
mod binary_operation;
mod conditional;
mod elementary_type_name_expression;
mod function_call;
mod function_call_options;
mod identifier;
mod index_access;
mod index_range_access;
mod literal;
mod member_access;
mod new_expression;
mod prelude;
mod tuple_expression;
mod unary_operation;

use crate::ast_specs::TypeDescriptions;
use serde::Deserialize;

pub use prelude::*;

crate::enum_refs! {
    #[derive_owned(Deserialize)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[serde(tag = "nodeType")]
    pub enum Expression {
        Assignment(Assignment),
        BinaryOperation(BinaryOperation),
        Conditional(Conditional),
        ElementaryTypeNameExpression(ElementaryTypeNameExpression),
        FunctionCall(FunctionCall),
        FunctionCallOptions(FunctionCallOptions),
        Identifier(Identifier),
        IndexAccess(IndexAccess),
        IndexRangeAccess(IndexRangeAccess),
        Literal(Literal),
        MemberAccess(MemberAccess),
        NewExpression(NewExpression),
        TupleExpression(TupleExpression),
        UnaryOperation(UnaryOperation),
    }
}

macro_rules! ex_delegate_functions {
    (@inner
        $enum_name:ident, $f_name:ident, $r_type:ty;
        $($variant:ident),*
    ) => {
        pub fn $f_name(&self) -> $r_type {
            match self {
                $(
                    $enum_name::$variant(i) => i.$f_name(),
                )*
            }
        }
    };
    (
        $($variant:ident),*
    ) => {
        impl Expression {
            ex_delegate_functions!(@inner Expression, id, isize; $($variant),*);
            ex_delegate_functions!(@inner Expression, src, &str; $($variant),*);
            ex_delegate_functions!(@inner Expression, type_descriptions, &TypeDescriptions; $($variant),*);
        }

        impl ExpressionRef<'_> {
            ex_delegate_functions!(@inner ExpressionRef, id, isize; $($variant),*);
            ex_delegate_functions!(@inner ExpressionRef, src, &str; $($variant),*);
            ex_delegate_functions!(@inner ExpressionRef, type_descriptions, &TypeDescriptions; $($variant),*);
        }
    };
}

ex_delegate_functions!(
    Assignment,
    BinaryOperation,
    Conditional,
    ElementaryTypeNameExpression,
    FunctionCall,
    FunctionCallOptions,
    Identifier,
    IndexAccess,
    IndexRangeAccess,
    Literal,
    MemberAccess,
    NewExpression,
    TupleExpression,
    UnaryOperation
);

impl Expression {
    pub fn extract_name(&self) -> String {
        match self {
            Expression::Identifier(identifier) => identifier.name().to_owned(),
            Expression::NewExpression(new_expression) => new_expression.name(),
            Expression::MemberAccess(member_access) => member_access.name(),
            Expression::FunctionCall(function_call) => function_call.full_name(),
            Expression::ElementaryTypeNameExpression(elementary_type_name_expression) => {
                elementary_type_name_expression.name().to_owned()
            }
            Expression::BinaryOperation(binary_operation) => binary_operation.as_name(),
            Expression::IndexAccess(index_access) => index_access.as_name(),
            Expression::Literal(literal) => literal.as_name().to_owned(),
            Expression::FunctionCallOptions(function_call_options) => {
                function_call_options.full_name()
            }
            _ => unimplemented!("{:?}", self),
        }
    }

    pub fn extract_definition(&self) -> Option<isize> {
        match self {
            Expression::Identifier(identifier) => identifier.referenced_declaration(),
            Expression::NewExpression(_) => None,
            Expression::MemberAccess(member_access) => member_access
                .referenced_declaration()
                .or(member_access.expression().extract_definition()),
            Expression::ElementaryTypeNameExpression(_) => None,
            Expression::FunctionCallOptions(fco) => fco.expression().extract_definition(),
            Expression::FunctionCall(fc) => fc.expression().extract_definition(),
            Expression::IndexAccess(ia) => ia.base_expression().extract_definition(),
            _ => unimplemented!("{:?}", self),
        }
    }

    pub fn is_builtin(&self) -> bool {
        match self {
            Expression::Assignment(_) => todo!(),
            Expression::BinaryOperation(_) => todo!(),
            Expression::Conditional(_) => todo!(),
            Expression::ElementaryTypeNameExpression(etne) => false,
            Expression::FunctionCall(fc) => fc.is_builtin(),
            Expression::FunctionCallOptions(fco) => fco.is_builtin(),
            Expression::Identifier(ident) => ident.is_builtin(),
            Expression::IndexAccess(_) => todo!(),
            Expression::IndexRangeAccess(_) => todo!(),
            Expression::Literal(_) => todo!(),
            Expression::MemberAccess(m_acc) => m_acc.is_builtin(),
            Expression::NewExpression(_) => todo!(),
            Expression::TupleExpression(_) => todo!(),
            Expression::UnaryOperation(_) => todo!(),
        }
    }
}
