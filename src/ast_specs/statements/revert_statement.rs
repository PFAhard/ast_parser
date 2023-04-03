use serde::Deserialize;

use crate::ast_specs::expressions::FunctionCall;

#[derive(Deserialize, Debug, Clone)]
pub struct RevertStatement {
    documentation: Option<String>,
    #[serde(rename = "errorCall")]
    error_call: FunctionCall,
    id: isize,
    src: String,
}

impl RevertStatement {
    pub fn error_call(&self) -> &FunctionCall {
        &self.error_call
    }

    pub fn id(&self) -> isize {
        self.id
    }
}