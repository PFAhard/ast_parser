use std::fmt::Display;

use getters::Getters;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct IdentifierPath {
    #[copy]
    id: isize,
    #[return_type = "&str"]
    name: String,
    #[serde(rename = "nameLocations")]
    name_locations: Option<Vec<String>>,
    #[serde(rename = "referencedDeclaration")]
    #[copy]
    referenced_declaration: isize,
    src: String,
}


impl Display for IdentifierPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}