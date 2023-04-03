use serde::Deserialize;

use crate::ast_parser::ast_specs::SourceUnit;


#[derive(Deserialize, Debug)]
pub(crate) struct FoundryOutput {
    ast: SourceUnit,
}

impl FoundryOutput {
    pub(crate) fn ast(&self) -> &SourceUnit {
        &self.ast
    }
}