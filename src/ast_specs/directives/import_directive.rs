use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::Identifier;

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct ImportDirective {
    #[serde(rename = "absolutePath")]
    absolute_path: String,
    file: String,
    id: isize,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
    scope: isize,
    #[serde(rename = "sourceUnit")]
    source_unit: isize,
    src: String,
    #[serde(rename = "symbolAliases")]
    symbol_aliases: Vec<SymbolAliases>,
    #[serde(rename = "unitAlias")]
    unit_alias: String,
}

#[derive(Deserialize, Debug, Clone, Getters)]
pub struct SymbolAliases {
    foreign: Identifier,
    local: Option<String>,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
}
