pub mod yul_literal_hex_value;
pub mod yul_literal_value;

use serde::{Deserialize, Serialize};
use yul_literal_hex_value::YulLiteralHexValue;
use yul_literal_value::YulLiteralValue;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum YulLiteral {
    YulLiteralValue(YulLiteralValue),
    YulLiteralHexValue(YulLiteralHexValue),
}
