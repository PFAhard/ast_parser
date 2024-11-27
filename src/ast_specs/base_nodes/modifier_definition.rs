use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::{
    Block, OverrideSpecifier, ParameterList, StructuredDocumentation, Visibility,
};

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct ModifierDefinition {
    #[serde(rename = "baseModifiers")]
    base_modifiers: Option<Vec<isize>>,
    body: Block,
    documentation: Option<StructuredDocumentation>,
    #[copy]
    id: isize,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    #[return_type = "Option<&OverrideSpecifier>"]
    #[use_as_ref]
    overrides: Option<OverrideSpecifier>,
    parameters: ParameterList,
    src: String,
    #[serde(rename = "virtual")]
    _virtual: bool,
    visibility: Visibility,
}
