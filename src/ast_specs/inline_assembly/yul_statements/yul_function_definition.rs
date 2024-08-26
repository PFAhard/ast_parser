use serde::{Deserialize, Serialize};

use crate::ast_specs::inline_assembly::yul_typed_name::YulTypedName;

use super::yul_block::YulBlock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YulFunctionDefinition {
    pub body: YulBlock,
    pub name: String,
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub parameters: Option<Vec<YulTypedName>>,
    pub return_variables: Option<Vec<YulTypedName>>,
    pub src: String,
}
