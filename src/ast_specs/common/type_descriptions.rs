use serde::Deserialize;


#[derive(Deserialize, Debug, Clone)]
pub struct TypeDescriptions {
    #[serde(rename = "typeIdentifier")]
    type_identifier: Option<String>,
    #[serde(rename = "typeString")]
    type_string: Option<String>,
}

impl TypeDescriptions {
    pub fn type_string(&self) -> Option<&String> {
        self.type_string.as_ref()
    }
}