use serde::Deserialize;

use crate::ast_parser::ast_specs::common::Visibility;

use super::prelude::VariableDeclaration;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct StructDefinition {
    #[serde(rename = "canonicalName")]
    canonical_name: String,
    id: isize,
    members: Vec<VariableDeclaration>,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    scope: isize,
    src: String,
    visibility: Visibility,
}

impl StructDefinition {
    pub(crate) fn id(&self) -> isize {
        self.id
    }

    pub(crate) fn members(&self) -> &[VariableDeclaration] {
        self.members.as_ref()
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) fn scope(&self) -> isize {
        self.scope
    }
}
