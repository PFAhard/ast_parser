use serde::Deserialize;

use crate::ast_specs::common::{TypeDescriptions, ElementaryTypeName};



#[derive(Deserialize, Debug, Clone)]
pub struct ElementaryTypeNameExpression {
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
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
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
    #[serde(rename = "typeName")]
    type_name: ElementaryTypeName,
}

impl ElementaryTypeNameExpression {
    pub fn name(&self) -> String {
        self.type_name().name()
    }

    pub fn type_name(&self) -> &ElementaryTypeName {
        &self.type_name
    }

    pub fn argument_types(&self) -> Option<&Vec<TypeDescriptions>> {
        self.argument_types.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }
}
