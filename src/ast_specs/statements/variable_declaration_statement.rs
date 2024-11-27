use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::{directives::VariableDeclaration, Expression};

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct VariableDeclarationStatement {
    #[return_type = "&[Option<isize>]"]
    assignments: Vec<Option<isize>>,
    #[return_type = "&[Option<VariableDeclaration>]"]
    declarations: Vec<Option<VariableDeclaration>>,
    documentation: Option<String>,
    #[copy]
    id: isize,
    #[serde(rename = "initialValue")]
    #[return_type = "Option<&Expression>"]
    #[use_as_ref]
    initial_value: Option<Expression>,
    src: String,
}