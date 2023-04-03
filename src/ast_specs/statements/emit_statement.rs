use serde::Deserialize;

use crate::ast_parser::ast_specs::expressions::FunctionCall;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct EmitStatement {
    documentation: Option<String>,
    #[serde(rename = "eventCall")]
    event_call: FunctionCall,
    id: isize,
    src: String,
}

impl EmitStatement {
    pub(crate) fn event_call(&self) -> &FunctionCall {
        &self.event_call
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}