use serde::Deserialize;

use crate::ast_specs::common::{StructuredDocumentation, ParameterList};


#[derive(Deserialize, Debug, Clone)]
pub struct ErrorDefinition {
    documentation: Option<StructuredDocumentation>,
    #[serde(rename = "errorSelector")]
    error_selector: Option<String>,
    id: isize,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    parameters: ParameterList,
    src: String,
}

impl ErrorDefinition {
    pub fn id(&self) -> isize {
        self.id
    }

    pub fn parameters(&self) -> &ParameterList {
        &self.parameters
    }
}
