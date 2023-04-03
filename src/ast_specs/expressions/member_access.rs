use serde::Deserialize;

use crate::ast_specs::common::TypeDescriptions;

use super::Expression;


#[derive(Deserialize, Debug, Clone)]
pub struct MemberAccess {
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
    expression: Box<Expression>,
    id: isize,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_lvalue: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    #[serde(rename = "memberLocation")]
    member_location: Option<String>,
    #[serde(rename = "memberName")]
    member_name: String,
    #[serde(rename = "referencedDeclaration")]
    referenced_declaration: Option<isize>,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl MemberAccess {
    pub fn expression(&self) -> &Expression {
        self.expression.as_ref()
    }

    pub fn name(&self) -> String {
        let base = self.expression().extract_name();
        let member = self.member_name();
        format!("{base}.{member}")
    }

    pub fn member_name(&self) -> &str {
        self.member_name.as_ref()
    }

    pub fn argument_types(&self) -> Option<&Vec<TypeDescriptions>> {
        self.argument_types.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }

    pub fn referenced_declaration(&self) -> Option<isize> {
        self.referenced_declaration
    }
}
