use serde::Deserialize;

use crate::ast_parser::ast_specs::Expression;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ExpressionStatement {
    documentation: Option<String>,
    expression: Expression,
    id: isize,
    src: String,
}

impl ExpressionStatement {
    pub(crate) fn expression(&self) -> &Expression {
        &self.expression
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}
