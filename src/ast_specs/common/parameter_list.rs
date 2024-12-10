use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::directives::VariableDeclaration;

#[derive(Deserialize, Debug, Clone, Getters, Default)]
pub struct ParameterList {
    #[copy]
    id: isize,
    #[return_type = "&[VariableDeclaration]"]
    parameters: Vec<VariableDeclaration>,
    src: String,
}

impl ParameterList {
    pub fn artificial_new(parameters: Vec<VariableDeclaration>) -> Self {
        Self {
            parameters,
            ..Default::default()
        }
    }
}
