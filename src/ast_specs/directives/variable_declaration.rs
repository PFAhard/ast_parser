use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::{
    common::{
        Mutability, OverrideSpecifier, StorageLocation, StructuredDocumentation, TypeDescriptions,
        TypeName, Visibility,
    },
    Expression,
};

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct VariableDeclaration {
    #[serde(rename = "baseFunctions")]
    base_functions: Option<Vec<isize>>,
    constant: bool,
    documentation: Option<StructuredDocumentation>,
    #[serde(rename = "functionSelector")]
    function_selector: Option<String>,
    id: isize,
    indexed: Option<bool>,
    #[copy]
    mutability: Option<Mutability>,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    overrides: Option<OverrideSpecifier>,
    scope: isize,
    src: String,
    #[serde(rename = "stateVariable")]
    #[copy]
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