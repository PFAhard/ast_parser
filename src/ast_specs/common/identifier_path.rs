use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct IdentifierPath {
    id: isize,
    name: String,
    #[serde(rename = "nameLocations")]
    name_locations: Option<Vec<String>>,
    #[serde(rename = "referencedDeclaration")]
    referenced_declaration: isize,
    src: String,
}

impl IdentifierPath {
    pub(crate) fn id(&self) -> isize {
        self.id
    }

    pub(crate) fn referenced_declaration(&self) -> isize {
        self.referenced_declaration
    }
}
