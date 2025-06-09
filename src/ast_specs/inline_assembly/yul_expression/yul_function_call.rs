use getters::Getters;
use serde::{Deserialize, Serialize};

use super::{yul_identifier::YulIdentifier, YulExpression};

#[derive(Debug, Serialize, Deserialize, Clone, Getters, PartialEq, Eq)]
pub struct YulFunctionCall {
    pub arguments: Vec<YulExpression>,
    #[serde(rename = "functionName")]
    pub function_name: YulIdentifier,
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
}
