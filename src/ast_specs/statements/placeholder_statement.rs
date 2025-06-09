use getters::Getters;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Getters, PartialEq, Eq)]
pub struct PlaceholderStatement {
    documentation: Option<String>,
    #[copy]
    id: isize,
    src: String,
}
