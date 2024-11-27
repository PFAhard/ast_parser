use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::Expression;

use super::BaseName;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct InheritanceSpecifier {
    arguments: Option<Vec<Expression>>,
    #[serde(rename = "baseName")]
    base_name: BaseName,
    id: isize,
    src: String,
}