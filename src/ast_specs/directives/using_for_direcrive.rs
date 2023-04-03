use serde::Deserialize;

use crate::ast_specs::common::{LibraryName, TypeName};


#[derive(Deserialize, Debug, Clone)]
pub struct UsingForDirective {
    #[serde(rename = "functionList")]
    function_list: Option<serde_json::Value>,
    global: Option<bool>,
    id: isize,
    #[serde(rename = "libraryName")]
    library_name: Option<LibraryName>,
    src: String,
    #[serde(rename = "typeName")]
    type_name: Option<TypeName>,
}

impl UsingForDirective {
    pub fn function_list(&self) -> Option<&serde_json::Value> {
        self.function_list.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }

    pub fn library_name(&self) -> Option<&LibraryName> {
        self.library_name.as_ref()
    }

    pub fn type_name(&self) -> Option<&TypeName> {
        self.type_name.as_ref()
    }
}