use getters::Getters;
use serde::Deserialize;

use super::{identifier_path::IdentifierPath, type_name::UserDefinedTypeName};

#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct OverrideSpecifier {
    #[copy]
    id: isize,
    #[return_type = "&[Overrides]"]
    overrides: Vec<Overrides>,
    src: String,
}


#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "nodeType")]
pub enum Overrides {
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
}
