use std::collections::HashMap;

use getters::Getters;
use serde::Deserialize;

use super::{
    Directive,
    directives::{ContractDefinition, Directives},
};

#[derive(Deserialize, Debug, Clone, Default, Getters, PartialEq, Eq)]
pub struct SourceUnit {
    #[serde(rename = "absolutePath")]
    #[return_type = "&str"]
    absolute_path: String,
    #[serde(rename = "exportedSymbols")]
    exported_symbols: HashMap<String, Vec<isize>>,
    #[copy]
    id: isize,
    license: Option<String>,
    #[use_as_ref]
    #[return_type = "&[Directive]"]
    nodes: Directives,
    #[return_type = "&str"]
    src: String,
}

impl SourceUnit {
    pub fn artificial_new(license: Option<String>, nodes: Directives) -> Self {
        Self {
            license,
            nodes,
            ..Default::default()
        }
    }

    pub fn is_in_exported_symbols(&self, id: isize) -> bool {
        self.exported_symbols
            .values()
            .any(|value| value.iter().any(|ex_id| ex_id == &id))
    }

    pub fn get_contract(&self, target_name: &str) -> &ContractDefinition {
        self.nodes()
            .iter()
            .filter_map(|node| match node {
                Directive::ContractDefinition(cd) => Some(cd),
                _ => None,
            })
            .find(|cd| cd.name() == target_name)
            .unwrap()
    }

    pub fn debug_nodes<'a>(&'a self) -> &'a [Directive] {
        &self.nodes
    }
}
