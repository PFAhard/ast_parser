use serde::Deserialize;

use super::{identifier_path::IdentifierPath, type_name::UserDefinedTypeName};

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct OverrideSpecifier {
    id: isize,
    overrides: Vec<Overrides>,
    src: String,
}

impl OverrideSpecifier {
    pub(crate) fn overrides(&self) -> &[Overrides] {
        self.overrides.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub(crate) enum Overrides {
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
}
