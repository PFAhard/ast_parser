use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct YulLiteralHexValue {
    #[serde(rename = "hexValue")]
    pub hex_value: String,
    pub kind: LiteralKind,
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
    pub r#type: String, // Use `r#type` to avoid conflict with Rust's `type` keyword
    #[serde(rename = "value")]
    pub value: Option<String>, // Optional field
}

// Reuse the LiteralKind enum from YulLiteralValue
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LiteralKind {
    String,
    Number,
    Bool,
}
