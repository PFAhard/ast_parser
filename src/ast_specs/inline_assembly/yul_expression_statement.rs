use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct YulExpressionStatement {
    pub expression: YulExpression,
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
}
