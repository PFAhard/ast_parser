use serde::Deserialize;

use crate::ast_parser::ast_specs::Expression;

use super::Body;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct DoWhileStatement {
    body: Box<Body>,
    condition: Expression,
    documentation: Option<String>,
    id: isize,
    src: String,
}

impl DoWhileStatement {
    pub(crate) fn body(&self) -> &Body {
        self.body.as_ref()
    }

    pub(crate) fn condition(&self) -> &Expression {
        &self.condition
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}