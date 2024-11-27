use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::{TypeDescriptions, TypeName};

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct NewExpression {
    #[serde(rename = "argumentTypes")]
    #[return_type = "Option<&Vec<TypeDescriptions>>"]
    #[use_as_ref]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[copy]
    id: isize,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_lvalue: Option<bool>,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "typeName")]
    type_name: TypeName,
}

impl NewExpression {
    pub fn name(&self) -> String {
        format!("new {}", self.type_name().name())
    }
}
