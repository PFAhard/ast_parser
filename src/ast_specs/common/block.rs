use serde::Deserialize;

use crate::ast_specs::Statement;

#[derive(Deserialize, Debug, Clone)]
pub struct Block {
    documentation: Option<String>,
    id: isize,
    src: String,
    statements: Option<Vec<Statement>>,
}

impl Block {
    pub fn statements(&self) -> Option<&[Statement]> {
        self.statements.as_deref()
    }

    pub fn src(&self) -> &str {
        self.src.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }
}