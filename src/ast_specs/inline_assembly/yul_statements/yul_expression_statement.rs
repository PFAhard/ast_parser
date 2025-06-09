use getters::Getters;
use serde::{Deserialize, Serialize};

use crate::ast_specs::inline_assembly::yul_expression::YulExpression;

#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct YulExpressionStatement {
    pub expression: YulExpression,
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
}
