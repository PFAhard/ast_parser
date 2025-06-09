use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::{
    common::{Block, ParameterList},
    expressions::FunctionCall,
};

#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct TryStatement {
    #[return_type = "&[TryCatchClause]"]
    clauses: Vec<TryCatchClause>,
    documentation: Option<String>,
    #[serde(rename = "externalCall")]
    external_call: FunctionCall,
    #[copy]
    id: isize,
    src: String,
}

#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct TryCatchClause {
    block: Block,
    #[serde(rename = "errorName")]
    error_name: String,
    #[copy]
    id: isize,
    #[return_type = "Option<&ParameterList>"]
    #[use_as_ref]
    parameters: Option<ParameterList>,
    src: String,
}