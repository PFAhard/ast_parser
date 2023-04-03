use serde::Deserialize;

use crate::ast_specs::{directives::VariableDeclaration, Expression};

#[derive(Deserialize, Debug, Clone)]
pub struct VariableDeclarationStatement {
    assignments: Vec<Option<isize>>,
    declarations: Vec<Option<VariableDeclaration>>,
    documentation: Option<String>,
    id: isize,
    #[serde(rename = "initialValue")]
    initial_value: Option<Expression>,
    src: String,
}

impl VariableDeclarationStatement {
    pub fn initial_value(&self) -> Option<&Expression> {
        self.initial_value.as_ref()
    }

    pub fn declarations(&self) -> &[Option<VariableDeclaration>] {
        self.declarations.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }
}