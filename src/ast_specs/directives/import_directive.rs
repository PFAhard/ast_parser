use serde::Deserialize;


#[derive(Deserialize, Debug, Clone)]
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
    symbol_aliases: serde_json::Value, // TODO: specify
    #[serde(rename = "unitAlias")]
    unit_alias: String,
}

impl ImportDirective {
    pub fn id(&self) -> isize {
        self.id
    }

    pub fn symbol_aliases(&self) -> &serde_json::Value {
        #[cfg(debug_assertions)]
        // eprintln!("[DEBUG] Symbol aliases are not yet supported");
        &self.symbol_aliases
    }

    pub fn source_unit(&self) -> isize {
        self.source_unit
    }
}
