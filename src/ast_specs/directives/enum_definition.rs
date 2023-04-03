use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct EnumDefinition {
    #[serde(rename = "canonicalName")]
    canonical_name: Option<String>,
    id: isize,
    members: Vec<EnumValue>,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    src: String,
}

impl EnumDefinition {
    pub(crate) fn members(&self) -> &[EnumValue] {
        self.members.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct EnumValue {
    id: isize,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    src: String,
}

impl EnumValue {
    pub(crate) fn id(&self) -> isize {
        self.id
    }
}
