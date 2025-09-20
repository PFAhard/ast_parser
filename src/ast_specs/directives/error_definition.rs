use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::{ParameterList, StructuredDocumentation};

#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct ErrorDefinition {
    documentation: Option<StructuredDocumentation>,
    #[serde(rename = "errorSelector")]
    error_selector: Option<String>,
    #[copy]
    id: isize,
    #[return_type = "&str"]
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    parameters: ParameterList,
    src: String,
}
