use serde::Deserialize;

use crate::ast_parser::ast_specs::{Expression, expressions::Identifier};

use super::identifier_path::IdentifierPath;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ModifierInvocation {
    arguments: Option<Vec<Expression>>,
    id: isize,
    kind: ModifierKind,
    #[serde(rename = "modifierName")]
    modifier_name: ModifierName,
    src: String,
}

impl ModifierInvocation {
    pub(crate) fn arguments(&self) -> Option<&Vec<Expression>> {
        self.arguments.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }

    pub(crate) fn modifier_name(&self) -> &ModifierName {
        &self.modifier_name
    }

    pub(crate) fn get_ref_id(&self) -> Option<isize> {
        match self.modifier_name() {
            ModifierName::Identifier(ident) => ident.referenced_declaration(),
            ModifierName::IdentifierPath(ident_path) => Some(ident_path.referenced_declaration()),
        }
    }

    pub(crate) fn src(&self) -> &str {
        self.src.as_ref()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) enum ModifierKind {
    #[serde(rename = "modifierInvocation")]
    ModifierInvocation,
    #[serde(rename = "baseConstructorSpecifier")]
    BaseConstructorSpecifier,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "nodeType")]
pub(crate) enum ModifierName {
    Identifier(Identifier),
    IdentifierPath(IdentifierPath),
}
