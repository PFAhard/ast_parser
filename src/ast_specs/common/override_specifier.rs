use serde::Deserialize;

use super::{identifier_path::IdentifierPath, type_name::UserDefinedTypeName};

#[derive(Deserialize, Debug, Clone)]
pub struct OverrideSpecifier {
    id: isize,
    overrides: Vec<Overrides>,
    src: String,
}

impl OverrideSpecifier {
    pub fn overrides(&self) -> &[Overrides] {
        self.overrides.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub enum Overrides {
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
}
