use serde::Deserialize;

use crate::ast_parser::ast_specs::common::TypeDescriptions;

use super::Expression;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Conditional {
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
    condition: Box<Expression>,
    #[serde(rename = "falseExpression")]
    false_expression: Box<Expression>,
    id: isize,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_lvalue: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    src: String,
    #[serde(rename = "trueExpression")]
    true_expression: Box<Expression>,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl Conditional {
    pub(crate) fn condition(&self) -> &Expression {
        self.condition.as_ref()
    }

    pub(crate) fn false_expression(&self) -> &Expression {
        self.false_expression.as_ref()
    }

    pub(crate) fn true_expression(&self) -> &Expression {
        self.true_expression.as_ref()
    }

    pub(crate) fn argument_types(&self) -> Option<&Vec<TypeDescriptions>> {
        self.argument_types.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}