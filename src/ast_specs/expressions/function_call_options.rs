use serde::Deserialize;

use crate::ast_parser::ast_specs::common::TypeDescriptions;

use super::Expression;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct FunctionCallOptions {
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
    expression: Box<Expression>,
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
    options: Vec<Expression>,
    src: String,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl FunctionCallOptions {
    pub(crate) fn expression(&self) -> &Expression {
        self.expression.as_ref()
    }

    pub(crate) fn options(&self) -> &[Expression] {
        self.options.as_ref()
    }

    pub(crate) fn argument_types(&self) -> Option<&Vec<TypeDescriptions>> {
        self.argument_types.as_ref()
    }

    pub(crate) fn id(&self) -> isize {
        self.id
    }

    pub(crate) fn full_name(&self) -> String {
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