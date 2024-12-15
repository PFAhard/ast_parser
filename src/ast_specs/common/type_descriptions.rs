use getters::Getters;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default, Getters)]
pub struct TypeDescriptions {
    #[serde(rename = "typeIdentifier")]
    #[use_as_deref]
    #[return_type = "Option<&str>"]
    type_identifier: Option<String>,
    #[serde(rename = "typeString")]
    #[use_as_deref]
    #[return_type = "Option<&str>"]
    type_string: Option<String>,
}

impl TypeDescriptions {
    pub fn artificial_new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
