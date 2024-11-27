use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::TypeDescriptions;

use super::Expression;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct MemberAccess {
    #[serde(rename = "argumentTypes")]
    #[return_type = "Option<&Vec<TypeDescriptions>>"]
    #[use_as_ref]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[return_type = "&Expression"]
    expression: Box<Expression>,
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
    #[serde(rename = "memberLocation")]
    member_location: Option<String>,
    #[serde(rename = "memberName")]
    #[return_type = "&str"]
    member_name: String,
    #[serde(rename = "referencedDeclaration")]
    #[copy]
    referenced_declaration: Option<isize>,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl MemberAccess {
    pub fn name(&self) -> String {
        let base = self.expression().extract_name();
        let member = self.member_name();
        format!("{base}.{member}")
    }

    pub fn is_builtin(&self) -> bool {
        self.expression().is_builtin()
    }
}
