use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct YulVariableDeclaration {
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
    #[serde(rename = "value")]
    pub value: Option<YulExpression>, // Handling `null` as `Option<YulExpression>`
    pub variables: Vec<YulTypedName>,
}
