use getters::Getters;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct EnumDefinition {
    #[serde(rename = "canonicalName")]
    canonical_name: Option<String>,
    #[copy]
    id: isize,
    #[return_type = "&[EnumValue]"]
    members: Vec<EnumValue>,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    src: String,
}

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct EnumValue {
    #[copy]
    id: isize,
    #[return_type = "&str"]
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    src: String,
}
 
