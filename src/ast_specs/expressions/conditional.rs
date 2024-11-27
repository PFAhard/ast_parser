use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::TypeDescriptions;

use super::Expression;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct Conditional {
    #[serde(rename = "argumentTypes")]
    #[use_as_ref]
    #[return_type = "Option<&Vec<TypeDescriptions>>"]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[return_type = "&Expression"]
    condition: Box<Expression>,
    #[serde(rename = "falseExpression")]
    #[return_type = "&Expression"]
    false_expression: Box<Expression>,
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
    #[serde(rename = "trueExpression")]
    #[return_type = "&Expression"]
    true_expression: Box<Expression>,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}
