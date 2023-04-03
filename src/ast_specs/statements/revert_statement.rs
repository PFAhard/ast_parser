use serde::Deserialize;

use crate::ast_parser::ast_specs::expressions::FunctionCall;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct RevertStatement {
    documentation: Option<String>,
    #[serde(rename = "errorCall")]
    error_call: FunctionCall,
    id: isize,
    src: String,
}

impl RevertStatement {
    pub(crate) fn error_call(&self) -> &FunctionCall {
        &self.error_call
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}