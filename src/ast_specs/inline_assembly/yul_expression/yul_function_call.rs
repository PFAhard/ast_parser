use serde::{Deserialize, Serialize};

use super::{yul_identifier::YulIdentifier, YulExpression};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YulFunctionCall {
    pub arguments: Vec<YulExpression>,
    pub function_name: YulIdentifier,
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
}
