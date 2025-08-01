use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::{common::Visibility, StructuredDocumentation};

use super::prelude::VariableDeclaration;

#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct StructDefinition {
    #[serde(rename = "canonicalName")]
    canonical_name: String,
    #[copy]
    id: isize,
    documentation: Option<StructuredDocumentation>,
    #[return_type = "&[VariableDeclaration]"]
    members: Vec<VariableDeclaration>,
    #[return_type = "&str"]
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    #[copy]
    scope: isize,
    #[return_type = "&str"]
    src: String,
    visibility: Visibility,
}
