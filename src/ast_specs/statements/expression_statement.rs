use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::Expression;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct ExpressionStatement {
    documentation: Option<String>,
    #[return_type = "&Option<Expression>"]
    expression: Option<Expression>,
    #[copy]
    id: isize,
    src: String,
}
