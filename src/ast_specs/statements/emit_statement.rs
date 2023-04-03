use serde::Deserialize;

use crate::ast_specs::expressions::FunctionCall;

#[derive(Deserialize, Debug, Clone)]
pub struct EmitStatement {
    documentation: Option<String>,
    #[serde(rename = "eventCall")]
    event_call: FunctionCall,
    id: isize,
    src: String,
}

impl EmitStatement {
    pub fn event_call(&self) -> &FunctionCall {
        &self.event_call
    }

    pub fn id(&self) -> isize {
        self.id
    }
}