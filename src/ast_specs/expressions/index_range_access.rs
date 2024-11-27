use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::TypeDescriptions;

use super::Expression;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct IndexRangeAccess {
    #[serde(rename = "argumentTypes")]
    #[return_type = "Option<&Vec<TypeDescriptions>>"]
    #[use_as_ref]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "baseExpression")]
    #[return_type = "&Expression"]
    base_expression: Box<Expression>,
    #[serde(rename = "endExpression")]
    #[return_type = "Option<&Expression>"]
    #[use_as_deref]
    end_expression: Option<Box<Expression>>,
    #[copy]
    id: isize,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_lvalue: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    src: String,
    #[serde(rename = "startExpression")]
    #[return_type = "Option<&Expression>"]
    #[use_as_deref]
    start_expression: Option<Box<Expression>>,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}
