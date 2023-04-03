use serde::Deserialize;

use crate::ast_parser::ast_specs::common::TypeDescriptions;

use super::Expression;


#[derive(Deserialize, Debug, Clone)]
pub(crate) struct TupleExpression {
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
    components: Vec<Expression>,
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
    pub(crate) fn components(&self) -> &[Expression] {
        self.components.as_ref()
    }

    pub(crate) fn argument_types(&self) -> Option<&Vec<TypeDescriptions>> {
        self.argument_types.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }
}