use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct YulAssignment {
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
    pub value: YulExpression,
    #[serde(rename = "variableNames")]
    pub variable_names: Vec<YulIdentifier>,
}
