use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::directives::VariableDeclaration;


#[derive(Deserialize, Debug, Clone, Getters)]
pub struct ParameterList {
    #[copy]
    id: isize,
    #[return_type = "&[VariableDeclaration]"]
    parameters: Vec<VariableDeclaration>,
    src: String,
}