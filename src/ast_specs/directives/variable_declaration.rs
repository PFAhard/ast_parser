use serde::Deserialize;

use crate::ast_specs::{
    common::{
        Mutability, OverrideSpecifier, StorageLocation, StructuredDocumentation, TypeDescriptions,
        TypeName, Visibility,
    },
    Expression,
};

#[derive(Deserialize, Debug, Clone)]
pub struct VariableDeclaration {
    #[serde(rename = "baseFunctions")]
    base_functions: Option<Vec<isize>>,
    constant: bool,
    documentation: Option<StructuredDocumentation>,
    #[serde(rename = "functionSelector")]
    function_selector: Option<String>,
    id: isize,
    indexed: Option<bool>,
    mutability: Mutability,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    overrides: Option<OverrideSpecifier>,
    scope: isize,
    src: String,
    #[serde(rename = "stateVariable")]
    state_variable: bool,
    #[serde(rename = "storageLocation")]
    storage_location: StorageLocation,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "typeName")]
    type_name: Option<TypeName>,
    value: Option<Expression>,
    visibility: Visibility,
}

impl VariableDeclaration {
    pub fn value(&self) -> Option<&Expression> {
        self.value.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }

    pub fn overrides(&self) -> Option<&OverrideSpecifier> {
        self.overrides.as_ref()
    }

    pub fn type_name(&self) -> Option<&TypeName> {
        self.type_name.as_ref()
    }

    pub fn scope(&self) -> isize {
        self.scope
    }
    
    pub fn state_variable(&self) -> bool {
        self.state_variable
    }
    
    pub fn visibility(&self) -> Visibility {
        self.visibility
    }
    
    pub fn mutability(&self) -> Mutability {
        self.mutability
    }
    
    pub fn constant(&self) -> bool {
        self.constant
    }
}
