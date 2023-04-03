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

use serde::Deserialize;

pub(crate) use prelude::*;

// @note
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub(crate) enum Expression {
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

impl Expression {
    pub(crate) fn extract_name(&self) -> String {
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

    pub(crate) fn extract_definition(&self) -> Option<isize> {
        match self {
            Expression::Identifier(identifier) => identifier.referenced_declaration(),
            Expression::NewExpression(_) => None,
            Expression::MemberAccess(member_access) => member_access.referenced_declaration(),
            Expression::ElementaryTypeNameExpression(_) => None,
            Expression::FunctionCallOptions(fco) => fco.expression().extract_definition(),
            _ => unimplemented!("{:?}", self),
        }
    }
}
