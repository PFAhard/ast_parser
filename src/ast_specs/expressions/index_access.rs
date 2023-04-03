use serde::Deserialize;

use crate::ast_specs::common::TypeDescriptions;

use super::Expression;

#[derive(Deserialize, Debug, Clone)]
pub struct IndexAccess {
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "baseExpression")]
    base_expression: Box<Expression>,
    id: isize,
    #[serde(rename = "indexExpression")]
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
    pub fn base_expression(&self) -> &Expression {
        self.base_expression.as_ref()
    }

    pub fn index_expression(&self) -> Option<&Expression> {
        self.index_expression.as_deref()
    }

    pub fn as_name(&self) -> String {
        let name = self.base_expression.extract_name();
        let index = match self.index_expression() {
            Some(index) => index.extract_name(),
            None => todo!(),
        };

        format!("{name}[{index}]")
    }

    pub fn argument_types(&self) -> Option<&Vec<TypeDescriptions>> {
        self.argument_types.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }
}
