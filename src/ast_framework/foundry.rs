use serde::Deserialize;

use crate::ast_specs::SourceUnit;


#[derive(Deserialize, Debug)]
pub struct FoundryOutput {
    ast: SourceUnit,
}

impl FoundryOutput {
    pub fn ast(&self) -> &SourceUnit {
        &self.ast
    }
}