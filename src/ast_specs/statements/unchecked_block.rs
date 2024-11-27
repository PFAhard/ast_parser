use getters::Getters;
use serde::Deserialize;

use super::Statement;


#[derive(Deserialize, Debug, Clone, Getters)]
pub struct UncheckedBlock {
    documentation: Option<String>,
    #[copy]
    id: isize,
    src: String,
    #[return_type = "&[Statement]"]
    statements: Vec<Statement>,
}
