use serde::Deserialize;

use crate::ast_parser::ast_specs::common::TypeDescriptions;

use super::Expression;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct UnaryOperation {
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
    function: Option<isize>,
    id: isize,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_lvalue: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    operator: String,
    prefix: bool,
    src: String,
    #[serde(rename = "subExpression")]
    sub_expression: Box<Expression>,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl UnaryOperation {
    pub(crate) fn sub_expression(&self) -> &Expression {
        self.sub_expression.as_ref()
    }

    pub(crate) fn argument_types(&self) -> Option<&Vec<TypeDescriptions>> {
        self.argument_types.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}
