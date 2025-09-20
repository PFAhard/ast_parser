use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::{ElementaryTypeName, TypeDescriptions};

#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct ElementaryTypeNameExpression {
    #[serde(rename = "argumentTypes")]
    #[use_as_ref]
    #[return_type = "Option<&Vec<TypeDescriptions>>"]
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
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "typeName")]
    type_name: CompatabilityTypeName,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum CompatabilityTypeName {
    ElementaryTypeName(ElementaryTypeName),
    Name(String),
}

impl CompatabilityTypeName {
    pub fn name(&self) -> &str {
        match self {
            CompatabilityTypeName::ElementaryTypeName(t) => t.name(),
            CompatabilityTypeName::Name(s) => s.as_str(),
        }
    }
}

impl ElementaryTypeNameExpression {
    pub fn name(&self) -> String {
        self.type_name().name().to_owned()
    }
}
