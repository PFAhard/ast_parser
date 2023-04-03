use std::collections::HashMap;

use serde::Deserialize;

use crate::ast_specs::SourceUnit;


#[derive(Deserialize, Debug)]
pub struct SolcOutput {
    // contracts: HashMap<String, serde_json::Value>,
    // #[serde(rename = "sourceList")]
    // source_list: Vec<String>,
    sources: HashMap<String, HashMap<String, SourceUnit>>,
    // version: String,
}

impl SolcOutput {
    pub fn ast(&self) -> &SourceUnit {
        let middle = self
            .sources
            .get(self.sources.keys().last().unwrap().as_str())
            .unwrap();
        middle.get(middle.keys().last().unwrap().as_str()).unwrap()
    }
}
