use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::TypeDescriptions;

use super::Expression;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct Assignment {
    #[serde(rename = "argumentTypes")]
    #[use_as_deref]
    #[return_type = "Option<&[TypeDescriptions]>"]
    argument_types: Option<Vec<TypeDescriptions>>,
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
    #[serde(rename = "leftHandSide")]
    #[return_type = "&Expression"]
    left_hand_side: Box<Expression>,
    #[return_type = "&str"]
    operator: String,
    #[serde(rename = "rightHandSide")]
    #[return_type = "&Expression"]
    right_hand_side: Box<Expression>,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}
