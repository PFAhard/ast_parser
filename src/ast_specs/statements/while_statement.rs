use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::Expression;

use super::Body;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct WhileStatement {
    #[return_type = "&Body"]
    body: Box<Body>,
    #[return_type = "Option<&Expression>"]
    #[use_as_ref]
    condition: Option<Expression>,
    documentation: Option<String>,
    #[copy]
    id: isize,
    src: String,
}
