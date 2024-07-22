use serde::Deserialize;

use crate::ast_specs::Expression;

#[derive(Deserialize, Debug, Clone)]
pub struct ExpressionStatement {
    documentation: Option<String>,
    expression: Option<Expression>,
    id: isize,
    src: String,
}

impl ExpressionStatement {
    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn id(&self) -> isize {
        self.id
    }
}
