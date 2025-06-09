use getters::Getters;
use serde::{Deserialize, Serialize};

use crate::ast_specs::inline_assembly::yul_expression::{yul_literal::YulLiteral, YulExpression};

use super::yul_block::YulBlock;

#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct YulSwitch {
    pub cases: Vec<YulCase>,
    pub expression: YulExpression,
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct YulCase {
    pub body: YulBlock,
    #[serde(rename = "nativeSrc")]
    pub native_src: Option<String>,
    pub src: String,
    pub value: CaseValue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum CaseValue {
    Default(String),
    YulLiteral(YulLiteral),
}
