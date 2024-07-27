use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YulLeave {
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
}
