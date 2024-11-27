use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::{LiteralKind, TypeDescriptions};

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct Literal {
    #[use_as_ref]
    #[serde(rename = "argumentTypes")]
    #[return_type = "Option<&Vec<TypeDescriptions>>"]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "hexValue")]
    hex_value: String,
    #[copy]
    id: isize,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_lvalue: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[copy]
    kind: LiteralKind,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    src: String,
    // #[serde(skip)]
    // subdenomination: (), // @note never seen
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[clone]
    value: Option<String>,
}

impl Literal {
    pub fn as_name(&self) -> &str {
        match &self.value {
            Some(value) => value,
            None => todo!(),
        }
    }
}
