use std::fmt::Display;

use serde::Deserialize;

use crate::ast_specs::common::{
    Block, ModifierInvocation, OverrideSpecifier, ParameterList, StateMutability,
    StructuredDocumentation, Visibility,
};

use super::prelude::VariableDeclaration;

#[derive(Deserialize, Debug, Clone)]
pub struct FunctionDefinition {
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
    pub fn body(&self) -> Option<&Block> {
        self.body.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn src(&self) -> &str {
        self.src.as_ref()
    }

    pub fn parameters(&self) -> Option<&[VariableDeclaration]> {
        self.parameters.as_ref().map(|p| p.parameters())
    }

    pub fn parameter_list(&self) -> Option<&ParameterList> {
        self.parameters.as_ref()
    }

    pub fn return_parameters(&self) -> Option<&[VariableDeclaration]> {
        self.return_parameters.as_ref().map(|r| r.parameters())
    }

    pub fn id(&self) -> isize {
        self.id
    }

    pub fn is_id(&self, id: isize) -> bool {
        self.id == id
    }

    pub fn modifiers(&self) -> &[ModifierInvocation] {
        self.modifiers.as_ref()
    }

    pub fn overrides(&self) -> Option<&OverrideSpecifier> {
        self.overrides.as_ref()
    }

    pub fn scope(&self) -> isize {
        self.scope
    }

    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    pub fn state_mutability(&self) -> StateMutability {
        self.state_mutability
    }

    pub fn full_name(&self) -> String {
        let mut name = self.name().to_owned();
        if let Some(params) = self.parameter_list() {
            let params: Vec<String> = params
                .parameters()
                .iter()
                .map(|x| format!("{} {}", x.type_name().as_ref().unwrap().name(), x.name()))
                .collect();

            let mut params = format!("{:?}", params);
            params.replace_range(0..1, "(");
            params.replace_range(params.len() - 1..params.len(), ")");
            name.push_str(params.as_str());
        }

        if let Some(returns) = self.return_parameters() {
            let returns: Vec<String> = returns
                .iter()
                .map(|x| format!("{} {}", x.type_name().as_ref().unwrap().name(), x.name()))
                .collect();
            if !returns.is_empty() {
                let mut returns = format!("{:?}", returns);
                returns.replace_range(0..1, "(");
                returns.replace_range(returns.len() - 1..returns.len(), ")");
                name.push_str("returns");
                name.push_str(returns.as_str());
            }
        }

        name
    }
    
    pub fn kind(&self) -> FunctionKind {
        self.kind
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum FunctionKind {
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

impl Display for FunctionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionKind::Function => f.write_str("function"),
            FunctionKind::Receive => f.write_str("receive"),
            FunctionKind::Constructor => f.write_str("constructor"),
            FunctionKind::Fallback => f.write_str("fallback"),
            FunctionKind::FreeFunction => f.write_str("freeFunction"),
        } 
    }
}