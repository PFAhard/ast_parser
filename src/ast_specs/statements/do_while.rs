use serde::Deserialize;

use crate::ast_specs::Expression;

use super::Body;

#[derive(Deserialize, Debug, Clone)]
pub struct DoWhileStatement {
    body: Box<Body>,
    condition: Option<Expression>,
    documentation: Option<String>,
    id: isize,
    src: String,
}

impl DoWhileStatement {
    pub fn body(&self) -> &Body {
        self.body.as_ref()
    }

    pub fn condition(&self) -> &Option<Expression> {
        &self.condition
    }

    pub fn id(&self) -> isize {
        self.id
    }
}