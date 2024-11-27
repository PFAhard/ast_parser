use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::TypeDescriptions;

use super::Expression;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct BinaryOperation {
    #[serde(rename = "argumentTypes")]
    #[skip_getter]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "commonType")]
    common_type: TypeDescriptions,
    function: Option<isize>,
    #[copy]
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
    #[return_type = "&Expression"]
    left_expression: Box<Expression>,
    #[return_type = "&str"]
    operator: String,
    #[serde(rename = "rightExpression")]
    #[return_type = "&Expression"]
    right_expression: Box<Expression>,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl BinaryOperation {
    pub fn as_name(&self) -> String {
        let left = self.left_expression().extract_name();
        let right = self.right_expression().extract_name();
        let operator = self.operator();

        format!("{left} {operator} {right}")
    }

    pub fn argument_types(&self) -> Option<&Vec<TypeDescriptions>> {
        self.argument_types.as_ref()
    }
}
