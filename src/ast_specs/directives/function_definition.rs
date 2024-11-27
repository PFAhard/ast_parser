use std::fmt::Display;

use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::{
    Block, ModifierInvocation, OverrideSpecifier, ParameterList, StateMutability,
    StructuredDocumentation, Visibility,
};

use super::prelude::VariableDeclaration;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct FunctionDefinition {
    #[serde(rename = "baseFunctions")]
    base_functions: Option<Vec<isize>>,
    #[return_type = "Option<&Block>"]
    #[use_as_ref]
    body: Option<Block>,
    documentation: Option<StructuredDocumentation>,
    #[serde(rename = "functionSelector")]
    function_selector: Option<String>,
    #[copy]
    id: isize,
    implemented: bool,
    #[copy]
    kind: FunctionKind,
    #[return_type = "&[ModifierInvocation]"]
    modifiers: Vec<ModifierInvocation>,
    #[return_type = "&str"]
    name: String,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    #[return_type = "Option<&OverrideSpecifier>"]
    #[use_as_ref]
    overrides: Option<OverrideSpecifier>,
    #[return_type = "Option<&ParameterList>"]
    #[use_as_ref]
    parameters: Option<ParameterList>,
    #[serde(rename = "returnParameters")]
    #[return_type = "Option<&ParameterList>"]
    #[use_as_ref]
    return_parameters: Option<ParameterList>,
    #[copy]
    scope: isize,
    #[return_type = "&str"]
    src: String,
    #[serde(rename = "stateMutability")]
    #[copy]
    state_mutability: StateMutability,
    #[serde(rename = "virtual")]
    _virtual: bool,
    #[copy]
    visibility: Visibility,
}

impl FunctionDefinition {
    pub fn parameter_list(&self) -> Option<&[VariableDeclaration]> {
        self.parameters.as_ref().map(|p| p.parameters())
    }

    pub fn return_parameter_list(&self) -> Option<&[VariableDeclaration]> {
        self.return_parameters.as_ref().map(|r| r.parameters())
    }

    pub fn is_id(&self, id: isize) -> bool {
        self.id == id
    }

    pub fn full_name(&self) -> String {
        let mut name = self.name().to_owned();
        if let Some(params) = self.parameters() {
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

        if let Some(returns) = self.return_parameter_list() {
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
