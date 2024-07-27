use serde::{Deserialize, Serialize};

use super::YulStatement;


#[derive(Debug, Serialize, Deserialize)]
pub struct YulBlock {
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
    pub statements: Vec<YulStatement>,
}
