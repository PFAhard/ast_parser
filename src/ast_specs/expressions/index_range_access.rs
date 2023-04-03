use serde::Deserialize;

use crate::ast_specs::common::TypeDescriptions;

use super::Expression;


#[derive(Deserialize, Debug, Clone)]
pub struct IndexRangeAccess {
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "baseExpression")]
    base_expression: Box<Expression>,
    #[serde(rename = "endExpression")]
    end_expression: Option<Box<Expression>>,
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
    #[serde(rename = "startExpression")]
    start_expression: Option<Box<Expression>>,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl IndexRangeAccess {
    pub fn base_expression(&self) -> &Expression {
        self.base_expression.as_ref()
    }

    pub fn end_expression(&self) -> Option<&Expression> {
        self.end_expression.as_deref()
    }

    pub fn start_expression(&self) -> Option<&Expression> {
        self.start_expression.as_deref()
    }

    pub fn argument_types(&self) -> Option<&Vec<TypeDescriptions>> {
        self.argument_types.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }
}