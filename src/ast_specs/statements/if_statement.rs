use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::Expression;

use super::FalseBody;

#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct IfStatement {
    #[use_as_ref]
    #[return_type = "Option<&Expression>"]
    condition: Option<Expression>,
    documentation: Option<String>,
    #[serde(rename = "falseBody")]
    #[return_type = "Option<&FalseBody>"]
    #[use_as_deref]
    false_body: Option<Box<FalseBody>>,
    #[copy]
    id: isize,
    src: String,
    #[serde(rename = "trueBody")]
    true_body: Box<FalseBody>, // TODO: Made it right
}
