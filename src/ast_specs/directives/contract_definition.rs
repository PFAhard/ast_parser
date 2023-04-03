use serde::Deserialize;

use crate::ast_parser::ast_specs::{
    common::{InheritanceSpecifier, StructuredDocumentation},
    BaseNode,
};

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ContractDefinition {
    #[serde(rename = "abstract")]
    _abstract: bool,
    #[serde(rename = "baseContracts")]
    base_contracts: Vec<InheritanceSpecifier>,
    #[serde(rename = "canonicalName")]
    canonical_name: Option<String>,
    #[serde(rename = "contractDependencies")]
    contract_dependencies: Vec<isize>,
    #[serde(rename = "contractKind")]
    contract_kind: ContractKind,
    documentation: Option<StructuredDocumentation>,
    #[serde(rename = "fullyImplemented")]
    fully_implemented: bool,
    id: isize,
    #[serde(rename = "linearizedBaseContracts")]
    linearized_base_contracts: Vec<isize>,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    nodes: Vec<BaseNode>,
    scope: isize,
    src: String,
    #[serde(rename = "usedErrors")]
    used_errors: Vec<isize>,
}

impl ContractDefinition {
    pub(crate) fn nodes(&self) -> &[BaseNode] {
        self.nodes.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }

    pub(crate) fn is_in_used_errors(&self, id: isize) -> bool {
        self.used_errors.iter().any(|ex_id| ex_id == &id)
    }

    pub(crate) fn used_errors(&self) -> &[isize] {
        self.used_errors.as_ref()
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) fn contract_kind(&self) -> &ContractKind {
        &self.contract_kind
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) enum ContractKind {
    #[serde(rename = "contract")]
    Contract,
    #[serde(rename = "interface")]
    Interface,
    #[serde(rename = "library")]
    Library,
}
