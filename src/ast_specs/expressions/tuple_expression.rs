use serde::Deserialize;

use crate::ast_specs::common::TypeDescriptions;

use super::Expression;


#[derive(Deserialize, Debug, Clone)]
pub struct TupleExpression {
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
    components: Vec<Option<Expression>>,
    id: isize,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isInlineArray")]
    is_inline_array: bool,
    #[serde(rename = "isLValue")]
    is_lvalue: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl TupleExpression {
    pub fn components(&self) -> &[Option<Expression>] {
        self.components.as_ref()
    }

    pub fn argument_types(&self) -> Option<&Vec<TypeDescriptions>> {
        self.argument_types.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }
}