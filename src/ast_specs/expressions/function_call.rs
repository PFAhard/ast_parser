use serde::Deserialize;

use crate::ast_specs::common::{FunctionCallKind, TypeDescriptions};

use super::Expression;

#[derive(Deserialize, Debug, Clone)]
pub struct FunctionCall {
    #[serde(rename = "argumentTypes")]
    argument_types: Option<Vec<TypeDescriptions>>,
    arguments: Vec<Expression>,
    expression: Box<Expression>,
    id: isize,
    #[serde(rename = "isConstant")]
    is_constant: bool,
    #[serde(rename = "isLValue")]
    is_lvalue: bool,
    #[serde(rename = "isPure")]
    is_pure: bool,
    kind: FunctionCallKind,
    #[serde(rename = "lValueRequested")]
    l_value_requested: bool,
    #[serde(rename = "nameLocations")]
    name_locations: Option<Vec<String>>,
    names: Vec<String>,
    src: String,
    #[serde(rename = "tryCall")]
    try_call: bool,
    #[serde(rename = "typeDescriptions")]
    type_descriptions: TypeDescriptions,
}

impl FunctionCall {
    pub fn arguments(&self) -> &[Expression] {
        self.arguments.as_ref()
    }

    pub fn expression(&self) -> &Expression {
        self.expression.as_ref()
    }

    pub fn argument_names(&self) -> Vec<String> {
        self.arguments()
            .iter()
            .map(Expression::extract_name)
            .collect()
    }

    pub fn src(&self) -> &str {
        self.src.as_ref()
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

    pub fn argument_types(&self) -> Option<&Vec<TypeDescriptions>> {
        self.argument_types.as_ref()
    }

    pub fn id(&self) -> isize {
        self.id
    }

    pub fn kind(&self) -> &FunctionCallKind {
        &self.kind
    }

    pub fn builtin(&self) -> Option<BuiltinFunction> {
        BuiltinFunction::try_from(self.expression().extract_definition().unwrap_or(0)).ok()
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
            return Ok(unsafe { std::mem::transmute::<isize, BuiltinFunction>(-value) });
        } else {
            Err(())
        }
    }
}
