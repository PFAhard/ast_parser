use serde::Deserialize;

use crate::ast_specs::{expressions::FunctionCall, common::{Block, ParameterList}};

#[derive(Deserialize, Debug, Clone)]
pub struct TryStatement {
    clauses: Vec<TryCatchClause>,
    documentation: Option<String>,
    #[serde(rename = "externalCall")]
    external_call: FunctionCall,
    id: isize,
    src: String,
}

impl TryStatement {
    pub fn clauses(&self) -> &[TryCatchClause] {
        self.clauses.as_ref()
    }

    pub fn external_call(&self) -> &FunctionCall {
        &self.external_call
    }

    pub fn id(&self) -> isize {
        self.id
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TryCatchClause {
    block: Block,
    #[serde(rename = "errorName")]
    error_name: String,
    id: isize,
    parameters: Option<ParameterList>,
    src: String,
}

impl TryCatchClause {
    pub fn block(&self) -> &Block {
        &self.block
    }

    pub fn id(&self) -> isize {
        self.id
    }

    pub fn parameters(&self) -> Option<&ParameterList> {
        self.parameters.as_ref()
    }
}
