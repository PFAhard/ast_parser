use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::StructuredDocumentation;

#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct EnumDefinition {
    documentation: Option<StructuredDocumentation>,
    #[serde(rename = "canonicalName")]
    canonical_name: Option<String>,
    #[copy]
    id: isize,
    #[return_type = "&[EnumValue]"]
    members: Vec<EnumValue>,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    src: String,
}

#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct EnumValue {
    #[copy]
    id: isize,
    #[return_type = "&str"]
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    src: String,
}
