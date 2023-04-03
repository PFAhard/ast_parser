use serde::Deserialize;

use crate::ast_specs::Expression;

use super::FalseBody;


#[derive(Deserialize, Debug, Clone)]
pub struct IfStatement {
    condition: Expression,
    documentation: Option<String>,
    #[serde(rename = "falseBody")]
    false_body: Option<Box<FalseBody>>,
    id: isize,
    src: String,
    #[serde(rename = "trueBody")]
    true_body: Box<FalseBody>, // TODO: Made it right
}

impl IfStatement {
    pub fn condition(&self) -> &Expression {
        &self.condition
    }

    pub fn false_body(&self) -> Option<&FalseBody> {
        self.false_body.as_deref()
    }

    pub fn true_body(&self) -> &FalseBody {
        self.true_body.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }
}