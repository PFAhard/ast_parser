use serde::Deserialize;

use crate::ast_parser::ast_specs::common::{StructuredDocumentation, ParameterList};


#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ErrorDefinition {
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
    pub(crate) fn id(&self) -> isize {
        self.id
    }

    pub(crate) fn parameters(&self) -> &ParameterList {
        &self.parameters
    }
}
