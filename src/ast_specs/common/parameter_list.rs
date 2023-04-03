use serde::Deserialize;

use crate::ast_specs::directives::VariableDeclaration;


#[derive(Deserialize, Debug, Clone)]
pub struct ParameterList {
    id: isize,
    parameters: Vec<VariableDeclaration>,
    src: String,
}

impl ParameterList {
    pub fn parameters(&self) -> &[VariableDeclaration] {
        self.parameters.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }
}
