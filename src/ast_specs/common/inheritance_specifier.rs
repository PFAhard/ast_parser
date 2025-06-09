use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::Expression;

use super::BaseName;

#[derive(Deserialize, Debug, Clone, Getters, Default, PartialEq, Eq)]
pub struct InheritanceSpecifier {
    arguments: Option<Vec<Expression>>,
    #[serde(rename = "baseName")]
    base_name: BaseName,
    id: isize,
    src: String,
}

impl InheritanceSpecifier {
    pub fn artificial_new(arguments: Option<Vec<Expression>>, base_name: BaseName) -> Self {
        Self {
            arguments,
            base_name,
            ..Default::default()
        }
    }
}
