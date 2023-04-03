use serde::Deserialize;

use super::Statement;


#[derive(Deserialize, Debug, Clone)]
pub(crate) struct UncheckedBlock {
    documentation: Option<String>,
    id: isize,
    src: String,
    statements: Vec<Statement>,
}

impl UncheckedBlock {
    pub(crate) fn statements(&self) -> &[Statement] {
        self.statements.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}
