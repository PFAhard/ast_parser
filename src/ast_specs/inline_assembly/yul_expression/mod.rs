pub mod yul_function_call;
pub mod yul_identifier;
pub mod yul_literal;

use serde::{Deserialize, Serialize};
use yul_function_call::YulFunctionCall;
use yul_identifier::YulIdentifier;
use yul_literal::YulLiteral;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "nodeType")]
pub enum YulExpression {
    YulFunctionCall(YulFunctionCall),
    YulIdentifier(YulIdentifier),
    YulLiteral(YulLiteral),
}

macro_rules! impl_from_variant {
    ($enum_name:ident, $variant:ident, $inner:ty) => {
        impl From<$inner> for $enum_name {
            fn from(val: $inner) -> Self {
                $enum_name::$variant(val)
            }
        }

        impl From<&$inner> for $enum_name {
            fn from(val: &$inner) -> Self {
                Self::from(val.to_owned())
            }
        }
    };
}

impl_from_variant!(YulExpression, YulFunctionCall, YulFunctionCall);
impl_from_variant!(YulExpression, YulIdentifier, YulIdentifier);
impl_from_variant!(YulExpression, YulLiteral, YulLiteral);
