use serde::Deserialize;


#[derive(Deserialize, Debug, Clone)]
pub(crate) struct TypeDescriptions {
    #[serde(rename = "typeIdentifier")]
    type_identifier: Option<String>,
    #[serde(rename = "typeString")]
    type_string: Option<String>,
}