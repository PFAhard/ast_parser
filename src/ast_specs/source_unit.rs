use std::collections::HashMap;

use serde::Deserialize;

use super::{directives::{Directives, ContractDefinition}, Directive};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct SourceUnit {
    #[serde(rename = "absolutePath")]
    absolute_path: String,
    #[serde(rename = "exportedSymbols")]
    exported_symbols: HashMap<String, Vec<isize>>,
    id: isize,
    license: Option<String>,
    // must be "SourceUnit"
    nodes: Directives,
    src: String,
}

impl SourceUnit {
    pub fn nodes(&self) -> &[Directive] {
        self.nodes.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }

    pub fn is_in_exported_symbols(&self, id: isize) -> bool {
        self.exported_symbols
            .values()
            .any(|value| value.iter().any(|ex_id| ex_id == &id))
    }

    pub fn get_contract(&self, target_name: &str) -> &ContractDefinition {
        self.nodes()
            .iter()
            .filter_map(|node| match node { Directive::ContractDefinition(cd) => Some(cd), _ => None })
            .find(|cd| cd.name() == target_name)
            .unwrap()
    }
    
    pub fn absolute_path(&self) -> &str {
        &self.absolute_path
    }
}
