use serde::Deserialize;

use crate::ast_specs::Expression;

use super::Body;

#[derive(Deserialize, Debug, Clone)]
pub struct WhileStatement {
    body: Box<Body>,
    condition: Expression,
    documentation: Option<String>,
    id: isize,
    src: String,
}

impl WhileStatement {
    pub fn body(&self) -> &Body {
        self.body.as_ref()
    }

    pub fn condition(&self) -> &Expression {
        &self.condition
    }

    pub fn id(&self) -> isize {
        self.id
    }
}
