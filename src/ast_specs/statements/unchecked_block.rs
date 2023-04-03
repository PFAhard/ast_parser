use serde::Deserialize;

use super::Statement;


#[derive(Deserialize, Debug, Clone)]
pub struct UncheckedBlock {
    documentation: Option<String>,
    id: isize,
    src: String,
    statements: Vec<Statement>,
}

impl UncheckedBlock {
    pub fn statements(&self) -> &[Statement] {
        self.statements.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }
}
