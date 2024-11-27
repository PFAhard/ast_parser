use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::expressions::FunctionCall;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct RevertStatement {
    documentation: Option<String>,
    #[serde(rename = "errorCall")]
    error_call: FunctionCall,
    #[copy]
    id: isize,
    src: String,
}
