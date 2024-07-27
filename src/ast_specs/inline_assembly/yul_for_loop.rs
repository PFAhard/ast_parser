use serde::{Deserialize, Serialize};

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
    