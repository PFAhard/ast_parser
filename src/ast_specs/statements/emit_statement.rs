use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::expressions::FunctionCall;

#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct EmitStatement {
    documentation: Option<String>,
    #[serde(rename = "eventCall")]
    event_call: FunctionCall,
    #[copy]
    id: isize,
    src: String,
}
