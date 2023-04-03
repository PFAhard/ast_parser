use serde::Deserialize;

use crate::ast_parser::ast_specs::common::TypeDescriptions;

use super::Expression;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct BinaryOperation {
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "commonType")]
    common_type: TypeDescriptions,
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
    #[serde(rename = "leftExpression")]
    left_expression: Box<Expression>,
    operator: String,
    #[serde(rename = "rightExpression")]
    right_expression: Box<Expression>,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl BinaryOperation {
    pub(crate) fn left_expression(&self) -> &Expression {
        self.left_expression.as_ref()
    }

    pub(crate) fn right_expression(&self) -> &Expression {
        self.right_expression.as_ref()
    }

    pub(crate) fn as_name(&self) -> String {
        let left = self.left_expression().extract_name();
        let right = self.right_expression().extract_name();
        let operator = self.operator();

        format!("{left} {operator} {right}")
    }

    pub(crate) fn operator(&self) -> &str {
        self.operator.as_ref()
    }

    pub(crate) fn argument_types(&self) -> Option<&Vec<TypeDescriptions>> {
        self.argument_types.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}
