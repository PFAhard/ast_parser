use serde::Deserialize;

use crate::ast_parser::ast_specs::{expressions::FunctionCall, common::{Block, ParameterList}};

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct TryStatement {
    clauses: Vec<TryCatchClause>,
    documentation: Option<String>,
    #[serde(rename = "externalCall")]
    external_call: FunctionCall,
    id: isize,
    src: String,
}

impl TryStatement {
    pub(crate) fn clauses(&self) -> &[TryCatchClause] {
        self.clauses.as_ref()
    }

    pub(crate) fn external_call(&self) -> &FunctionCall {
        &self.external_call
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct TryCatchClause {
    block: Block,
    #[serde(rename = "errorName")]
    error_name: String,
    id: isize,
    parameters: Option<ParameterList>,
    src: String,
}

impl TryCatchClause {
    pub(crate) fn block(&self) -> &Block {
        &self.block
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }

    pub(crate) fn parameters(&self) -> Option<&ParameterList> {
        self.parameters.as_ref()
    }
}
