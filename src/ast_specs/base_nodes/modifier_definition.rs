use serde::Deserialize;

use crate::ast_specs::common::{
    OverrideSpecifier, ParameterList, StructuredDocumentation, Visibility, Block,
};

#[derive(Deserialize, Debug, Clone)]
pub struct ModifierDefinition {
    #[serde(rename = "baseModifiers")]
    base_modifiers: Option<Vec<isize>>,
    body: Block,
    documentation: Option<StructuredDocumentation>,
    id: isize,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    overrides: Option<OverrideSpecifier>,
    parameters: ParameterList,
    src: String,
    #[serde(rename = "virtual")]
    _virtual: bool,
    visibility: Visibility,
}

impl ModifierDefinition {
    pub fn body(&self) -> &Block {
        &self.body
    }

    pub fn overrides(&self) -> Option<&OverrideSpecifier> {
        self.overrides.as_ref()
    }

    pub fn parameters(&self) -> &ParameterList {
        &self.parameters
    }

    pub fn id(&self) -> isize {
        self.id
    }
}
