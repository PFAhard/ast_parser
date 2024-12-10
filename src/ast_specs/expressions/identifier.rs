use std::fmt::Display;

use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::TypeDescriptions;

#[derive(Deserialize, Debug, Clone, Getters, Default)]
pub struct Identifier {
    #[serde(rename = "argumentTypes")]
    #[use_as_ref]
    #[return_type = "Option<&Vec<TypeDescriptions>>"]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[copy]
    id: isize,
    #[return_type = "&str"]
    name: String,
    #[serde(rename = "overloadedDeclarations")]
    overloaded_declarations: Vec<isize>,
    #[serde(rename = "referencedDeclaration")]
    #[copy]
    referenced_declaration: Option<isize>,
    #[return_type = "&str"]
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl Identifier {
    pub fn artificial_new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    pub fn is_builtin(&self) -> bool {
        matches!(self.referenced_declaration(), Some(x) if x < 0)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
