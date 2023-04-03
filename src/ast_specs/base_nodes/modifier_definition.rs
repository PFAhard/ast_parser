use serde::Deserialize;

use crate::ast_parser::ast_specs::common::{
    OverrideSpecifier, ParameterList, StructuredDocumentation, Visibility, Block,
};

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ModifierDefinition {
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
    pub(crate) fn body(&self) -> &Block {
        &self.body
    }

    pub(crate) fn overrides(&self) -> Option<&OverrideSpecifier> {
        self.overrides.as_ref()
    }

    pub(crate) fn parameters(&self) -> &ParameterList {
        &self.parameters
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}
