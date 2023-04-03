use serde::Deserialize;

use crate::ast_parser::ast_specs::directives::VariableDeclaration;


#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ParameterList {
    id: isize,
    parameters: Vec<VariableDeclaration>,
    src: String,
}

impl ParameterList {
    pub(crate) fn parameters(&self) -> &[VariableDeclaration] {
        self.parameters.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}
