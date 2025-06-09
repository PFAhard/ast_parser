use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::TypeName;

#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct UserDefinedValueTypeDefinition {
    #[serde(rename = "canonicalName")]
    canonical_name: Option<String>,
    #[copy]
    id: isize,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    src: String,
    #[serde(rename = "underlyingType")]
    underlying_type: TypeName,
}