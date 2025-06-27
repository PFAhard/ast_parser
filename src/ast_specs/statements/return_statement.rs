use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::Expression;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Getters)]
pub struct Return {
    documentation: Option<String>,
    #[return_type = "Option<&Expression>"]
    #[use_as_ref]
    expression: Option<Expression>,
    #[serde(rename = "functionReturnParameters")]
    #[copy]
    function_return_parameters: Option<isize>,
    #[copy]
    id: isize,
    src: String,
}
