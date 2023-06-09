use serde::Deserialize;

use crate::ast_specs::common::Visibility;

use super::prelude::VariableDeclaration;

#[derive(Deserialize, Debug, Clone)]
pub struct StructDefinition {
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
    pub fn id(&self) -> isize {
        self.id
    }

    pub fn members(&self) -> &[VariableDeclaration] {
        self.members.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn scope(&self) -> isize {
        self.scope
    }
}
