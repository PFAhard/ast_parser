use serde::Deserialize;

use crate::ast_specs::Expression;

#[derive(Deserialize, Debug, Clone)]
pub struct Return {
    documentation: Option<String>,
    expression: Option<Expression>,
    #[serde(rename = "functionReturnParameters")]
    function_return_parameters: isize,
    id: isize,
    src: String,
}

impl Return {
    pub fn expression(&self) -> Option<&Expression> {
        self.expression.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }

    pub fn function_return_parameters(&self) -> isize {
        self.function_return_parameters
    }
}