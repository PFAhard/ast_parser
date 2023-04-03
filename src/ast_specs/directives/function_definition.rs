use serde::Deserialize;

use crate::ast_parser::ast_specs::common::{
    Block, ModifierInvocation, OverrideSpecifier, ParameterList, StateMutability,
    StructuredDocumentation, Visibility,
};

use super::prelude::VariableDeclaration;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct FunctionDefinition {
    #[serde(rename = "baseFunctions")]
    base_functions: Option<Vec<isize>>,
    body: Option<Block>,
    documentation: Option<StructuredDocumentation>,
    #[serde(rename = "functionSelector")]
    function_selector: Option<String>,
    id: isize,
    implemented: bool,
    kind: FunctionKind,
    modifiers: Vec<ModifierInvocation>,
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    overrides: Option<OverrideSpecifier>,
    parameters: Option<ParameterList>,
    #[serde(rename = "returnParameters")]
    return_parameters: Option<ParameterList>,
    scope: isize,
    src: String,
    #[serde(rename = "stateMutability")]
    state_mutability: StateMutability,
    #[serde(rename = "virtual")]
    _virtual: bool,
    visibility: Visibility,
}

impl FunctionDefinition {
    pub(crate) fn body(&self) -> Option<&Block> {
        self.body.as_ref()
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) fn src(&self) -> &str {
        self.src.as_ref()
    }

    pub(crate) fn parameters(&self) -> Option<&[VariableDeclaration]> {
        self.parameters.as_ref().map(|p| p.parameters())
    }

    pub(crate) fn return_parameters(&self) -> Option<&[VariableDeclaration]> {
        self.return_parameters.as_ref().map(|r| r.parameters())
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }

    pub(crate) fn is_id(&self, id: isize) -> bool {
        self.id == id
    }

    pub(crate) fn modifiers(&self) -> &[ModifierInvocation] {
        self.modifiers.as_ref()
    }

    pub(crate) fn overrides(&self) -> Option<&OverrideSpecifier> {
        self.overrides.as_ref()
    }

    pub(crate) fn scope(&self) -> isize {
        self.scope
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) enum FunctionKind {
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "receive")]
    Receive,
    #[serde(rename = "constructor")]
    Constructor,
    #[serde(rename = "fallback")]
    Fallback,
    #[serde(rename = "freeFunction")]
    FreeFunction,
}
