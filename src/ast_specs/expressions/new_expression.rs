use serde::Deserialize;

use crate::ast_specs::common::{TypeDescriptions, TypeName};

#[derive(Deserialize, Debug, Clone)]
pub struct NewExpression {
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
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

    pub fn type_name(&self) -> &TypeName {
        &self.type_name
    }

    pub fn argument_types(&self) -> Option<&Vec<TypeDescriptions>> {
        self.argument_types.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }
}