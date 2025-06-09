use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::TypeDescriptions;

use super::Expression;

#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct IndexAccess {
    #[serde(rename = "argumentTypes")]
    #[use_as_ref]
    #[return_type = "Option<&Vec<TypeDescriptions>>"]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "baseExpression")]
    #[return_type = "&Expression"]
    base_expression: Box<Expression>,
    #[copy]
    id: isize,
    #[serde(rename = "indexExpression")]
    #[use_as_deref]
    #[return_type = "Option<&Expression>"]
    index_expression: Option<Box<Expression>>,
    #[serde(rename = "isConstant")]
    is_constant: bool,
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

impl IndexAccess {
    pub fn as_name(&self) -> String {
        let name = self.base_expression.extract_name();
        let index = match self.index_expression() {
            Some(index) => index.extract_name(),
            None => todo!(),
        };

        format!("{name}[{index}]")
    }
}
