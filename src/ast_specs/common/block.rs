use serde::Deserialize;

use crate::ast_parser::ast_specs::Statement;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Block {
    documentation: Option<String>,
    id: isize,
    src: String,
    statements: Option<Vec<Statement>>,
}

impl Block {
    pub(crate) fn statements(&self) -> Option<&[Statement]> {
        self.statements.as_deref()
    }

    pub(crate) fn src(&self) -> &str {
        self.src.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}