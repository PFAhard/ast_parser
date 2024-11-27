use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::{
    common::{InheritanceSpecifier, StructuredDocumentation},
    BaseNode,
};

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct ContractDefinition {
    #[serde(rename = "abstract")]
    #[copy]
    _abstract: bool,
    #[serde(rename = "baseContracts")]
    base_contracts: Vec<InheritanceSpecifier>,
    #[serde(rename = "canonicalName")]
    canonical_name: Option<String>,
    #[serde(rename = "contractDependencies")]
    contract_dependencies: Vec<isize>,
    #[serde(rename = "contractKind")]
    #[copy]
    contract_kind: ContractKind,
    documentation: Option<StructuredDocumentation>,
    #[serde(rename = "fullyImplemented")]
    fully_implemented: bool,
    #[copy]
    id: isize,
    #[serde(rename = "linearizedBaseContracts")]
    #[return_type = "&[isize]"]
    linearized_base_contracts: Vec<isize>,
    #[return_type = "&str"]
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    #[return_type = "&[BaseNode]"]
    nodes: Vec<BaseNode>,
    scope: isize,
    src: String,
    #[serde(rename = "usedErrors")]
    #[return_type = "&[isize]"]
    used_errors: Vec<isize>,
}

impl ContractDefinition {
    pub fn is_in_used_errors(&self, id: isize) -> bool {
        self.used_errors.iter().any(|ex_id| ex_id == &id)
    }
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractKind {
    #[serde(rename = "contract")]
    Contract,
    #[serde(rename = "interface")]
    Interface,
    #[serde(rename = "library")]
    Library,
}
