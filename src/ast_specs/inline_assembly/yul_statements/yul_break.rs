use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct YulBreak {
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
}
