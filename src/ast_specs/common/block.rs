use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::Statement;

#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct Block {
    documentation: Option<String>,
    #[copy]
    id: isize,
    #[return_type = "&str"]
    src: String,
    #[use_as_deref]
    #[return_type = "Option<&[Statement]>"]
    statements: Option<Vec<Statement>>,
}

