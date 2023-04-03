use serde::Deserialize;

use crate::ast_parser::ast_specs::Expression;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Return {
    documentation: Option<String>,
    expression: Option<Expression>,
    #[serde(rename = "functionReturnParameters")]
    function_return_parameters: isize,
    id: isize,
    src: String,
}

impl Return {
    pub(crate) fn expression(&self) -> Option<&Expression> {
        self.expression.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}