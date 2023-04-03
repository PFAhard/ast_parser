use serde::Deserialize;

use crate::ast_specs::common::TypeDescriptions;

#[derive(Deserialize, Debug, Clone)]
pub struct Identifier {
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
    id: isize,
    name: String,
    #[serde(rename = "overloadedDeclarations")]
    overloaded_declarations: Vec<isize>,
    #[serde(rename = "referencedDeclaration")]
    referenced_declaration: Option<isize>,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl Identifier {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn referenced_declaration(&self) -> Option<isize> {
        self.referenced_declaration
    }

    pub fn src(&self) -> &str {
        self.src.as_ref()
    }

    pub fn argument_types(&self) -> Option<&Vec<TypeDescriptions>> {
        self.argument_types.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }
}
