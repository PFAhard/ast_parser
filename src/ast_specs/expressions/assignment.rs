use serde::Deserialize;

use crate::ast_parser::ast_specs::common::TypeDescriptions;

use super::Expression;


#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Assignment {
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
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
    left_hand_side: Box<Expression>,
    operator: String,
    #[serde(rename = "rightHandSide")]
    right_hand_side: Box<Expression>,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl Assignment {
    pub(crate) fn left_hand_side(&self) -> &Expression {
        self.left_hand_side.as_ref()
    }

    pub(crate) fn right_hand_side(&self) -> &Expression {
        self.right_hand_side.as_ref()
    }

    pub(crate) fn argument_types(&self) -> Option<&Vec<TypeDescriptions>> {
        self.argument_types.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}