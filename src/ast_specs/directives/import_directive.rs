use getters::Getters;
use serde::Deserialize;

use crate::ast_specs::Identifier;

#[derive(Deserialize, Debug, Clone, Getters, Default)]
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

impl ImportDirective {
    pub fn artificial_new(file: String, symbol_aliases: Vec<SymbolAliases>) -> Self {
        Self {
            file,
            symbol_aliases,
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Debug, Clone, Getters, Default)]
pub struct SymbolAliases {
    foreign: Identifier,
    #[use_as_deref]
    #[return_type = "Option<&str>"]
    local: Option<String>,
    #[serde(rename = "nameLocation")]
    name_location: Option<String>,
}

impl SymbolAliases {
    pub fn artificial_new(foreign: Identifier, local: Option<String>) -> Self {
        Self {
            foreign,
            local,
            ..Default::default()
        }
    }
}
