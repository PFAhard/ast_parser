use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::TypeDescriptions;

use super::Expression;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct FunctionCallOptions {
    #[serde(rename = "argumentTypes")]
    #[use_as_ref]
    #[return_type = "Option<&Vec<TypeDescriptions>>"]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[return_type = "&Expression"]
    expression: Box<Expression>,
    #[copy]
    id: isize,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_lvalue: Option<bool>,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    names: Vec<String>,
    #[return_type = "&[Expression]"]
    options: Vec<Expression>,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl FunctionCallOptions {
    pub fn full_name(&self) -> String {
        let name = self.expression().extract_name();
        let args: Vec<String> = self
            .options()
            .iter()
            .map(|arg| arg.extract_name())
            .collect();
        let args = args.join(",");

        format!("{name}{{{args}}}")
    }
}