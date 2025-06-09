use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::{LibraryName, TypeName};

#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct UsingForDirective {
    #[serde(rename = "functionList")]
    #[use_as_ref]
    #[return_type = "Option<&serde_json::Value>"]
    function_list: Option<serde_json::Value>,
    global: Option<bool>,
    #[copy]
    id: isize,
    #[serde(rename = "libraryName")]
    #[return_type = "Option<&LibraryName>"]
    #[use_as_ref]
    library_name: Option<LibraryName>,
    src: String,
    #[serde(rename = "typeName")]
    #[return_type = "Option<&TypeName>"]
    #[use_as_ref]
    type_name: Option<TypeName>,
}
