use getters::Getters;
use serde::{Deserialize, Serialize};

use crate::ast_specs::common::{LiteralKind, TypeDescriptions};

#[derive(Deserialize, Serialize, Debug, Clone, Getters, PartialEq, Eq)]
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
    subdenomination: Option<Subdenomination>,
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Subdenomination {
    Weeks,
    Days,
    Hours,
    Minutes,
    Seconds,
    Wei,
    Gwei,
    Ether,
    Finney,
    Szabo,
}
