use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct YulContinue {
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
}
