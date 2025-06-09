use getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone, Default, Getters, PartialEq, Eq)]
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
