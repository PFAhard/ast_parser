use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YulTypedName {
    pub name: String,
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
    pub r#type: String, // Use `r#type` to avoid conflict with Rust's `type` keyword
}
