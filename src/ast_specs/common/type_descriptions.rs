use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct TypeDescriptions {
    #[serde(rename = "typeIdentifier")]
    type_identifier: Option<String>,
    #[serde(rename = "typeString")]
    type_string: Option<String>,
}

impl TypeDescriptions {
    pub fn artificial_new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn type_string(&self) -> Option<&String> {
        self.type_string.as_ref()
    }
}
