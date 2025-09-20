use std::fmt::Display;

use getters::Getters;
use serde::{Deserialize, Serialize};

use crate::ast_specs::{
    BaseNode,
    common::{InheritanceSpecifier, StructuredDocumentation},
};

#[derive(Deserialize, Debug, Clone, Getters, Default, PartialEq, Eq)]
pub struct ContractDefinition {
    #[serde(rename = "abstract")]
    #[copy]
    _abstract: Option<bool>,
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
    #[return_type = "&std::option::Option<Vec<isize>>"]
    used_errors: Option<Vec<isize>>,
}

impl ContractDefinition {
    pub fn artificial_new(
        _abstract: bool,
        base_contracts: Vec<InheritanceSpecifier>,
        contract_kind: ContractKind,
        name: String,
        nodes: Vec<BaseNode>,
    ) -> Self {
        Self {
            _abstract: Some(_abstract),
            base_contracts,
            contract_kind,
            name,
            nodes,
            ..Default::default()
        }
    }

    pub fn is_in_used_errors(&self, id: isize) -> bool {
        self.used_errors
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .any(|ex_id| ex_id == &id)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContractKind {
    #[serde(rename = "contract")]
    #[default]
    Contract,
    #[serde(rename = "interface")]
    Interface,
    #[serde(rename = "library")]
    Library,
}

impl Display for ContractKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractKind::Contract => write!(f, "contract"),
            ContractKind::Interface => write!(f, "interface"),
            ContractKind::Library => write!(f, "library"),
        }
    }
}
