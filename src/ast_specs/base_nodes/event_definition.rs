use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::{ParameterList, StructuredDocumentation};

#[derive(Deserialize, Debug, Clone, Getters, Default)]
pub struct EventDefinition {
    anonymous: bool,
    documentation: Option<StructuredDocumentation>,
    #[serde(rename = "eventSelector")]
    event_selector: Option<String>,
    #[copy]
    id: isize,
    #[return_type = "&str"]
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    parameters: ParameterList,
    #[return_type = "&str"]
    src: String,
}

impl EventDefinition {
    pub fn artificial_new(name: String, parameters: ParameterList) -> Self {
        Self {
            name,
            parameters,
            ..Default::default()
        }
    }
}
