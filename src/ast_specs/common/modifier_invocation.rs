use std::fmt::Display;

use serde::Deserialize;

use crate::ast_specs::{expressions::Identifier, Expression};

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

impl Display for ModifierName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModifierName::Identifier(id) => write!(f, "{}", id),
            ModifierName::IdentifierPath(ip) => write!(f, "{}", ip),
        }
    }
}
