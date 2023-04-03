use serde::Deserialize;

use crate::ast_specs::{Expression, expressions::Identifier};

use super::identifier_path::IdentifierPath;

#[derive(Deserialize, Debug, Clone)]
pub struct ModifierInvocation {
    arguments: Option<Vec<Expression>>,
    id: isize,
    kind: ModifierKind,
    #[serde(rename = "modifierName")]
    modifier_name: ModifierName,
    src: String,
}

impl ModifierInvocation {
    pub fn arguments(&self) -> Option<&Vec<Expression>> {
        self.arguments.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }

    pub fn modifier_name(&self) -> &ModifierName {
        &self.modifier_name
    }

    pub fn get_ref_id(&self) -> Option<isize> {
        match self.modifier_name() {
            ModifierName::Identifier(ident) => ident.referenced_declaration(),
            ModifierName::IdentifierPath(ident_path) => Some(ident_path.referenced_declaration()),
        }
    }

    pub fn src(&self) -> &str {
        self.src.as_ref()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum ModifierKind {
    #[serde(rename = "modifierInvocation")]
    ModifierInvocation,
    #[serde(rename = "baseConstructorSpecifier")]
    BaseConstructorSpecifier,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub enum ModifierName {
    Identifier(Identifier),
    IdentifierPath(IdentifierPath),
}
