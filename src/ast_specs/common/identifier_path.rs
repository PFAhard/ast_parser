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
}
