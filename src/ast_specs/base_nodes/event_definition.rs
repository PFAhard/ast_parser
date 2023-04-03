use serde::Deserialize;

use crate::ast_specs::common::{StructuredDocumentation, ParameterList};

#[derive(Deserialize, Debug, Clone)]
pub struct EventDefinition {
    anonymous: bool,
    documentation: Option<StructuredDocumentation>,
    #[serde(rename = "eventSelector")]
    event_selector: Option<String>,
    id: isize,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    parameters: ParameterList,
    src: String,
}

impl EventDefinition {
    pub fn parameters(&self) -> &ParameterList {
        &self.parameters
    }

    pub fn id(&self) -> isize {
        self.id
    }
}
