use getters::Getters;
use serde::{Deserialize, Serialize};

use crate::ast_specs::inline_assembly::yul_expression::{yul_identifier::YulIdentifier, YulExpression};

#[derive(Debug, Serialize, Deserialize, Clone, Getters, PartialEq, Eq)]
pub struct YulAssignment {
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
    pub value: YulExpression,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<YulIdentifier>,
}
