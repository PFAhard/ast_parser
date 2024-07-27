use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YulLiteralValue {
    pub kind: LiteralKind,
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
    pub r#type: String, // Use `r#type` to avoid conflict with Rust's `type` keyword
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LiteralKind {
    String,
    Number,
    Bool,
}
