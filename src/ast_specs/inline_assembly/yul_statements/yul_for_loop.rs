use serde::{Deserialize, Serialize};

use crate::ast_specs::inline_assembly::yul_expression::YulExpression;

use super::yul_block::YulBlock;

#[derive(Debug, Serialize, Deserialize)]
pub struct YulForLoop {
    pub body: YulBlock,
    pub condition: YulExpression,
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub post: YulBlock,
    pub pre: YulBlock,
    pub src: String,
}
    