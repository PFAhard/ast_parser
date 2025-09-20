use std::fmt::Display;

use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::{Expression, expressions::Identifier};

use super::identifier_path::IdentifierPath;

#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct ModifierInvocation {
    #[return_type = "Option<&Vec<Expression>>"]
    #[use_as_ref]
    arguments: Option<Vec<Expression>>,
    #[copy]
    id: isize,
    kind: Option<ModifierKind>,
    #[serde(rename = "modifierName")]
    modifier_name: ModifierName,
    #[return_type = "&str"]
    src: String,
}

impl ModifierInvocation {
    pub fn get_ref_id(&self) -> Option<isize> {
        match self.modifier_name() {
            ModifierName::Identifier(ident) => ident.referenced_declaration(),
            ModifierName::IdentifierPath(ident_path) => Some(ident_path.referenced_declaration()),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ModifierKind {
    #[serde(rename = "modifierInvocation")]
    ModifierInvocation,
    #[serde(rename = "baseConstructorSpecifier")]
    BaseConstructorSpecifier,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
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
