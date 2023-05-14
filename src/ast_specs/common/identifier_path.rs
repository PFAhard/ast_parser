use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct IdentifierPath {
    id: isize,
    name: String,
    #[serde(rename = "nameLocations")]
    name_locations: Option<Vec<String>>,
    #[serde(rename = "referencedDeclaration")]
    referenced_declaration: isize,
    src: String,
}

impl IdentifierPath {
    pub fn id(&self) -> isize {
        self.id
    }

    pub fn referenced_declaration(&self) -> isize {
        self.referenced_declaration
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

impl Display for IdentifierPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}