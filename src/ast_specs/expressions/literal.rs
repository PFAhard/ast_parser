use serde::Deserialize;

use crate::ast_parser::ast_specs::common::{TypeDescriptions, LiteralKind};


#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Literal {
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "hexValue")]
    hex_value: String,
    id: isize,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_lvalue: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    kind: LiteralKind,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    src: String,
    // #[serde(skip)]
    // subdenomination: (), // @note never seen
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    value: Option<String>,
}

impl Literal {
    pub(crate) fn argument_types(&self) -> Option<&Vec<TypeDescriptions>> {
        self.argument_types.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }

    pub(crate) fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    pub(crate) fn as_name(&self) -> &str {
        match &self.value {
            Some(value) => value,
            None => todo!(),
        }
    }
}