pub mod yul_function_call;
pub mod yul_identifier;
pub mod yul_literal;

use serde::{Deserialize, Serialize};
use yul_function_call::YulFunctionCall;
use yul_identifier::YulIdentifier;
use yul_literal::YulLiteral;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum YulExpression {
    YulFunctionCall(YulFunctionCall),
    YulIdentifier(YulIdentifier),
    YulLiteral(YulLiteral),
}
