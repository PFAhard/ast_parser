use serde::{Deserialize, Serialize};

use super::yul_block::YulBlock;

#[derive(Debug, Serialize, Deserialize)]
pub struct YulIf {
    pub body: YulBlock,
    pub condition: YulExpression,
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
}
