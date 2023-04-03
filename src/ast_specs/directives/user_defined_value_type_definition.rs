use serde::Deserialize;

use crate::ast_parser::ast_specs::common::TypeName;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct UserDefinedValueTypeDefinition {
    #[serde(rename = "canonicalName")]
    canonical_name: Option<String>,
    id: isize,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    src: String,
    #[serde(rename = "underlyingType")]
    underlying_type: TypeName,
}

impl UserDefinedValueTypeDefinition {
    pub(crate) fn underlying_type(&self) -> &TypeName {
        &self.underlying_type
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}