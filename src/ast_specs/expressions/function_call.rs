use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::common::{FunctionCallKind, TypeDescriptions};

use super::Expression;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct FunctionCall {
    #[serde(rename = "argumentTypes")]
    #[use_as_ref]
    #[return_type = "Option<&Vec<TypeDescriptions>>"]
    argument_types: Option<Vec<TypeDescriptions>>,
    #[use_as_ref]
    #[return_type = "&[Expression]"]
    arguments: Vec<Expression>,
    #[return_type = "&Expression"]
    expression: Box<Expression>,
    #[copy]
    id: isize,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_lvalue: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    #[copy]
    kind: FunctionCallKind,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    #[serde(rename = "nameLocations")]
    name_locations: Option<Vec<String>>,
    names: Vec<String>,
    #[return_type = "&str"]
    src: String,
    #[serde(rename = "tryCall")]
    try_call: Option<bool>,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl FunctionCall {
    pub fn argument_names(&self) -> Vec<String> {
        self.arguments()
            .iter()
            .map(Expression::extract_name)
            .collect()
    }

    pub fn full_name(&self) -> String {
        let name = self.expression().extract_name();
        let args: Vec<String> = self
            .arguments()
            .iter()
            .map(|arg| arg.extract_name())
            .collect();
        let args = args.join(",");

        format!("{name}({args})")
    }

    pub fn extract_function_definition_id(&self) -> isize {
        let expression = self.expression();
        expression.extract_definition().unwrap_or(-1)
    }

    pub fn builtin(&self) -> Option<BuiltinFunction> {
        BuiltinFunction::try_from(self.expression().extract_definition().unwrap_or(0)).ok()
    }

    pub fn is_builtin(&self) -> bool {
        self.expression().is_builtin()
    }
}

#[repr(usize)]
#[derive(Debug, Clone)]
pub enum BuiltinFunction {
    Require = 18,
}

impl TryFrom<isize> for BuiltinFunction {
    type Error = ();

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value < 0 {
            Ok(unsafe { std::mem::transmute::<isize, BuiltinFunction>(-value) })
        } else {
            Err(())
        }
    }
}
