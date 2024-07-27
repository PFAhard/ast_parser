use serde::{Deserialize, Serialize};

use crate::ast_specs::inline_assembly::{yul_expression::YulExpression, yul_typed_name::YulTypedName};

#[derive(Debug, Serialize, Deserialize)]
pub struct YulVariableDeclaration {
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
    #[serde(rename = "value")]
    pub value: Option<YulExpression>, // Handling `null` as `Option<YulExpression>`
    pub variables: Vec<YulTypedName>,
}
